use super::dbmodels::{DBChannel, DBParentMessage, DBReply, DBUser, DBSearchResult};
use crate::env::EnvVars;
use crate::types::{Channel, Message, User, SearchResult};
use sqlx::{
    postgres::PgPoolOptions,
    query_as,
    types::chrono::{self, NaiveDateTime},
    PgPool,
};
use std::time::Duration;

#[derive(Clone)]
pub struct Tummy {
    tummy_conn_pool: PgPool,
}

pub(crate) trait SlackDateTime {
    fn human_format(&self) -> String;
    fn from_pg_ts(str: &str) -> Self;
}

impl SlackDateTime for NaiveDateTime {
    fn human_format(&self) -> String {
        self.format("%d %b %Y @ %I:%M %p").to_string()
    }

    fn from_pg_ts(str: &str) -> Self {
        println!("{}", str);
        Self::parse_from_str(str, "%Y-%m-%dT%H:%M:%S%.f").unwrap()
    }
}

impl Tummy {
    pub async fn init(env_vars: &EnvVars) -> Self {
        let tummy_conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            env_vars.tummy_username,
            env_vars.tummy_password,
            env_vars.tummy_host,
            env_vars.tummy_port,
            env_vars.tummy_db
        );

        let tummy_conn_pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&tummy_conn_string)
            .await
            .expect("Could not connect to tummy.");

        sqlx::migrate!("./migrations")
            .run(&tummy_conn_pool)
            .await
            .expect("Could not run tummy migrations.");

        Self { tummy_conn_pool }
    }

    pub async fn get_all_channels(&self) -> color_eyre::Result<Vec<Channel>> {
        let db_channels = query_as!(DBChannel, "SELECT * FROM channels ORDER BY name ASC")
            .fetch_all(&self.tummy_conn_pool)
            .await?;

        Ok(db_channels.into_iter().map(Channel::from).collect())
    }

    pub async fn get_all_users(&self) -> color_eyre::Result<Vec<User>> {
        let db_channels = query_as!(DBUser, "SELECT * FROM users ORDER BY name ASC")
            .fetch_all(&self.tummy_conn_pool)
            .await?;

        Ok(db_channels.into_iter().map(User::from).collect())
    }

    pub async fn get_channel_info(&self, channel_id: &str) -> Result<Channel, sqlx::Error> {
        let channel = query_as!(
            DBChannel,
            "SELECT * FROM channels WHERE id = $1",
            channel_id
        )
            .fetch_one(&self.tummy_conn_pool)
            .await?;
        Ok(channel.into())
    }

    pub async fn search_msg_text(
        &self,
        query_text: &str,
        channel_id: Option<&str>,
        user_id: Option<&str>,
        limit: i64,
        before: Option<NaiveDateTime>,
        after: Option<NaiveDateTime>,
    ) -> color_eyre::Result<Vec<SearchResult>> {
        println!("RRF Search with User: {:?}, Channel: {:?}", user_id, channel_id);

        let is_text_search = !query_text.trim().is_empty();
        if !is_text_search {
            let mut builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
                r#"
            SELECT
                m.channel_id,
                c.name AS channel_name,
                m.user_id,
                m.msg_text,
                m.ts,
                m.thread_ts,
                m.parent_user_id,
                u.id,
                u.name,
                u.real_name,
                u.display_name,
                u.image_url,
                u.email,
                u.deleted,
                u.is_bot,
                (SELECT COUNT(*) FROM messages WHERE thread_ts = m.ts) as cnt,
                NULL as parent_msg_text,
                NULL as parent_name,
                NULL as parent_real_name,
                NULL as parent_display_name,
                NULL as parent_image_url,
                NULL as parent_email,
                NULL as parent_deleted,
                NULL as parent_is_bot
            FROM
                messages m
            INNER JOIN
                users u ON u.id = m.user_id
            INNER JOIN
                channels c ON c.id = m.channel_id
            "#,
            );
            builder.push(" WHERE m.parent_user_id IS NULL OR m.parent_user_id = ''");

            if let Some(cid) = channel_id {
                builder.push(" AND m.channel_id = ");
                builder.push_bind(cid);
            }
            if let Some(uid) = user_id {
                builder.push(" AND m.user_id = ");
                builder.push_bind(uid);
            }
            if let Some(bef) = before {
                builder.push(" AND m.ts < ");
                builder.push_bind(bef);
            }
            if let Some(after) = after {
                builder.push(" AND m.ts > ");
                builder.push_bind(after);
            }

            builder.push(" ORDER BY m.ts DESC LIMIT ");
            builder.push_bind(limit);

            let query = builder.build_query_as::<crate::db::dbmodels::DBSearchResult>();
            let recent_messages = query.fetch_all(&self.tummy_conn_pool).await?;

            return Ok(recent_messages.into_iter().map(SearchResult::from).collect());
        }

        // sanitize for tsquery
        let sanitized_query_text = query_text
            .replace(":", " ")
            .replace("&", " ")
            .replace("|", " ")
            .replace("!", " ")
            .replace("(", " ")
            .replace(")", " ");

        // split words
        let search_terms: Vec<&str> = sanitized_query_text.split_whitespace().collect();
        let full_text_query = search_terms.join(" & ");

        // applies the prefix search operator only to the last term
        let partial_text_query = if search_terms.len() > 1 {
            let mut partial_query = search_terms[0..search_terms.len() - 1].join(" & ");
            partial_query.push_str(&format!(" & {}:*", search_terms.last().unwrap()));
            partial_query
        } else {
            format!("{}:*", search_terms.first().unwrap_or(&""))
        };

        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("WITH ");

        // Fuzzy
        builder.push(r#"
    fuzzy AS (
        SELECT
            ts,
            similarity(msg_text, "#);
        builder.push_bind(query_text);
        builder.push(r#") as sim_score,
            row_number() OVER (ORDER BY similarity(msg_text, "#);
        builder.push_bind(query_text);
        builder.push(r#") DESC) as rank_ix
        FROM messages
        WHERE msg_text %> "#);
        builder.push_bind(query_text);
        if let Some(cid) = channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(cid);
        }
        if let Some(uid) = user_id {
            builder.push(" AND user_id = ");
            builder.push_bind(uid);
        }
        if let Some(bef) = before {
            builder.push(" AND ts < ");
            builder.push_bind(bef);
        }
        if let Some(after) = after {
            builder.push(" AND ts > ");
            builder.push_bind(after);
        }
        builder.push(r#"
        ORDER BY rank_ix
        LIMIT 30
    ),
"#);

        // Full text
        builder.push(r#"
    full_text AS (
        SELECT
            ts,
            ts_rank_cd(msg_tsv, to_tsquery('english', "#);
        builder.push_bind(&full_text_query);
        builder.push(r#")) as rank_score,
            row_number() OVER (ORDER BY ts_rank_cd(msg_tsv, to_tsquery('english', "#);
        builder.push_bind(&full_text_query);
        builder.push(r#")) DESC) as rank_ix
        FROM messages
        WHERE msg_tsv @@ to_tsquery('english', "#);
        builder.push_bind(&full_text_query);
        builder.push(r#")"#);
        if let Some(cid) = channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(cid);
        }
        if let Some(uid) = user_id {
            builder.push(" AND user_id = ");
            builder.push_bind(uid);
        }
        if let Some(bef) = before {
            builder.push(" AND ts < ");
            builder.push_bind(bef);
        }
        if let Some(after) = after {
            builder.push(" AND ts > ");
            builder.push_bind(after);
        }
        builder.push(r#"
        ORDER BY rank_ix
        LIMIT 30
    ),
"#);

        // Prefix search
        builder.push(r#"
    partial_search AS (
        SELECT
            ts,
            ts_rank_cd(msg_tsv, to_tsquery('simple', "#);
        builder.push_bind(&partial_text_query);
        builder.push(r#")) as rank_score,
            row_number() OVER (ORDER BY ts_rank_cd(msg_tsv, to_tsquery('simple', "#);
        builder.push_bind(&partial_text_query);
        builder.push(r#")) DESC) as rank_ix
        FROM messages
        WHERE msg_tsv @@ to_tsquery('simple', "#);
        builder.push_bind(&partial_text_query);
        builder.push(r#")"#);
        if let Some(cid) = channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(cid);
        }
        if let Some(uid) = user_id {
            builder.push(" AND user_id = ");
            builder.push_bind(uid);
        }
        if let Some(bef) = before {
            builder.push(" AND ts < ");
            builder.push_bind(bef);
        }
        if let Some(after) = after {
            builder.push(" AND ts > ");
            builder.push_bind(after);
        }
        builder.push(r#"
        LIMIT 30
    )
"#);

        // RRF
        builder.push(r#"
    SELECT
        m.channel_id, channel.name AS channel_name, m.user_id, m.msg_text, m.ts, m.thread_ts, m.parent_user_id,
        u.id, u.name, u.real_name, u.display_name, u.image_url, u.email, u.deleted, u.is_bot,
        c.cnt,
        CASE WHEN parent_u.id IS NOT NULL THEN parent_m.msg_text ELSE NULL END as "parent_msg_text",
        CASE WHEN parent_u.is_bot IS NOT NULL THEN parent_u.name ELSE NULL END as "parent_name",
        CASE WHEN parent_u.id IS NOT NULL THEN parent_u.real_name ELSE NULL END as "parent_real_name",
        CASE WHEN parent_u.id IS NOT NULL THEN parent_u.display_name ELSE NULL END as "parent_display_name",
        CASE WHEN parent_u.id IS NOT NULL THEN parent_u.image_url ELSE NULL END as "parent_image_url",
        CASE WHEN parent_u.id IS NOT NULL THEN parent_u.email ELSE NULL END as "parent_email",
        CASE WHEN parent_u.id IS NOT NULL THEN parent_u.deleted ELSE NULL END as "parent_deleted",
        CASE WHEN parent_u.id IS NOT NULL THEN parent_u.is_bot ELSE NULL END as "parent_is_bot"
    FROM
        fuzzy
        FULL OUTER JOIN full_text ON fuzzy.ts = full_text.ts
        FULL OUTER JOIN partial_search ON COALESCE(fuzzy.ts, full_text.ts) = partial_search.ts
        JOIN messages m ON COALESCE(fuzzy.ts, full_text.ts, partial_search.ts) = m.ts
        INNER JOIN users AS u ON u.id = m.user_id
        INNER JOIN channels AS channel ON channel.id = m.channel_id
        LEFT JOIN (SELECT COUNT(*) as cnt, thread_ts FROM messages WHERE parent_user_id != '' GROUP BY thread_ts) AS c ON m.thread_ts = c.thread_ts
        LEFT JOIN messages AS parent_m ON m.thread_ts = parent_m.ts AND parent_m.parent_user_id = ''
        LEFT JOIN users AS parent_u ON parent_m.user_id = parent_u.id
    ORDER BY
        -- Reciprocal Rank Fusion (RRF)
        -- The k value (e.g., 60) and weights can be tuned.
        COALESCE(1.0 / (60 + fuzzy.rank_ix), 0.0) * 1.0 +
        COALESCE(1.0 / (60 + full_text.rank_ix), 0.0) * 1.0 +
        COALESCE(1.0 / (60 + partial_search.rank_ix), 0.0) * 1.0
        DESC
"#);

        builder.push(" LIMIT ");
        builder.push_bind(limit);

        let query = builder.build_query_as::<DBSearchResult>();
        let messages = query.fetch_all(&self.tummy_conn_pool).await?;

        Ok(messages.into_iter().map(SearchResult::from).collect())
    }



    pub async fn fetch_replies(
        &self,
        message_ts: &str,
        channel_id: &str,
        user_id: &str,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let replies = query_as!(
            DBReply,
            r#"
            SELECT
                m.channel_id,
                c.name AS channel_name,
                m.user_id,
                m.msg_text,
                m.ts,
                m.thread_ts,
                m.parent_user_id,
                u.id,
                u.name,
                u.real_name,
                u.display_name,
                u.image_url,
                u.email,
                u.deleted,
                u.is_bot
            FROM
                messages AS m
            INNER JOIN users AS u ON u.id = m.user_id
            INNER JOIN channels AS c ON c.id = m.channel_id
            WHERE
                m.thread_ts = $1 AND m.channel_id = $2 AND m.parent_user_id = $3
            ORDER BY
                m.ts ASC
            "#,
            chrono::NaiveDateTime::from_pg_ts(message_ts),
            channel_id,
            user_id
        )
            .fetch_all(&self.tummy_conn_pool)
            .await?;
        Ok(replies.into_iter().map(Message::from).collect())
    }

    pub async fn fetch_msg_page(
        &self,
        channel_id: &str,
        before_msg_timestamp: &Option<chrono::NaiveDateTime>,
        msgs_per_page: &u32,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let fetched_messages = if let Some(timestamp) = before_msg_timestamp {
            // This is the backward pagination case.
            // We fetch messages that have a timestamp BEFORE the provided `before_msg_timestamp`.
            // The `ORDER BY ts DESC` ensures we get the most recent messages of the
            // older batch first.
            query_as!(
            DBParentMessage,
            r#"
            SELECT
                m.channel_id,
                ch.name AS channel_name, -- Added the channel name here
                m.user_id,
                m.msg_text,
                m.ts,
                m.thread_ts,
                m.parent_user_id,
                u.id,
                u.name,
                u.real_name,
                u.display_name,
                u.image_url,
                u.email,
                u.deleted,
                u.is_bot,
                c.cnt
            FROM
                messages AS m
            INNER JOIN users AS u ON u.id = m.user_id
            INNER JOIN channels AS ch ON ch.id = m.channel_id -- Joined the channels table
            LEFT JOIN (
                SELECT
                    COUNT(*) as cnt,
                    thread_ts as join_ts,
                    parent_user_id
                FROM
                    messages
                WHERE
                    channel_id = $1
                GROUP BY
                    join_ts,
                    parent_user_id
            ) AS c ON m.ts = c.join_ts AND m.user_id = c.parent_user_id
            WHERE
                m.channel_id = $1 AND m.ts < $2 AND m.parent_user_id = ''
            ORDER BY
                ts DESC
            LIMIT $3
            "#,
            channel_id,
            timestamp,
            *msgs_per_page as i64
        )
                .fetch_all(&self.tummy_conn_pool)
                .await?
        } else {
            // This is the initial load case.
            // We fetch the most recent messages from the channel.
            // `ORDER BY ts DESC` gets the latest messages, and `LIMIT` gets a single page.
            query_as!(
            DBParentMessage,
            "
            SELECT
                m.channel_id,
                ch.name AS channel_name, -- Added the channel name here
                m.user_id,
                m.msg_text,
                m.ts,
                m.thread_ts,
                m.parent_user_id,
                u.id,
                u.name,
                u.real_name,
                u.display_name,
                u.image_url,
                u.email,
                u.deleted,
                u.is_bot,
                c.cnt
            FROM
                messages AS m
            INNER JOIN users AS u ON u.id = m.user_id
            INNER JOIN channels AS ch ON ch.id = m.channel_id -- Joined the channels table
            LEFT JOIN (
                SELECT
                    COUNT(*) as cnt,
                    thread_ts as join_ts,
                    parent_user_id
                FROM
                    messages
                WHERE
                    channel_id = $1
                GROUP BY
                    join_ts,
                    parent_user_id
            ) AS c ON m.ts = c.join_ts AND m.user_id = c.parent_user_id
            WHERE
                m.channel_id = $1 AND m.parent_user_id = ''
            ORDER BY
                m.ts DESC
            LIMIT $2
         ",
            channel_id,
            *msgs_per_page as i64
        )
                .fetch_all(&self.tummy_conn_pool)
                .await?
        };
        Ok(fetched_messages
            .into_iter().rev()
            .map(Message::from)
            .collect())
    }

    pub async fn get_user_info(&self, user_id: &str) -> Result<User, sqlx::Error> {
        let user = query_as!(DBUser, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&self.tummy_conn_pool)
            .await?;
        Ok(user.into())
    }
}
