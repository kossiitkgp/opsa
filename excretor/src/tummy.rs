use sqlx::{
    postgres::PgPoolOptions,
    query_as,
    types::chrono::{self, NaiveDateTime},
    PgPool,
};
use std::time::Duration;

use crate::{
    dbmodels::{DBChannel, DBParentMessage, DBReply, DBUser},
    env::EnvVars,
    models::{self, Channel, Message, User},
};

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
        Self::parse_from_str(str, "%Y-%m-%d %X%.f").unwrap()
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

        Self {
            tummy_conn_pool: PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&tummy_conn_string)
                .await
                .expect("Could not connect to tummy."),
        }
    }

    pub async fn get_all_channels(&self) -> color_eyre::Result<Vec<Channel>> {
        let db_channels = query_as!(DBChannel, 
            "SELECT * FROM channels ORDER BY name ASC"
        )
            .fetch_all(&self.tummy_conn_pool)
            .await?;

        Ok(db_channels.into_iter().map(models::Channel::from).collect())
    }

    pub async fn get_channel_info(&self, channel_name: &str) -> Result<Channel, sqlx::Error> {
        let channel = query_as!(
            DBChannel,
            "SELECT * FROM channels WHERE name = $1",
            channel_name
        )
        .fetch_one(&self.tummy_conn_pool)
        .await?;
        Ok(channel.into())
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
            SELECT messages.*, users.*
            FROM messages
            INNER JOIN users ON users.id = messages.user_id
            WHERE thread_ts = $1 AND channel_id = $2 AND parent_user_id = $3
            ORDER BY ts ASC
            "#,
            chrono::NaiveDateTime::from_pg_ts(message_ts),
            channel_id,
            user_id
        )
        .fetch_all(&self.tummy_conn_pool)
        .await?;
        Ok(replies.into_iter().map(models::Message::from).collect())
    }

    pub async fn fetch_msg_page(
        &self,
        channel_id: &str,
        last_msg_timestamp: &Option<chrono::NaiveDateTime>,
        msgs_per_page: &u32,
        since_timestamp: &chrono::NaiveDateTime,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let fetched_messages = if let Some(timestamp) = last_msg_timestamp {
            query_as!(
                DBParentMessage,
                r#"
                SELECT messages.*, users.*, c.cnt
                FROM messages
                INNER JOIN users ON users.id = messages.user_id
                LEFT JOIN (
                    SELECT COUNT(*) as cnt, thread_ts as join_ts, parent_user_id
                    FROM messages
                    WHERE channel_id = $1
                    GROUP BY join_ts, parent_user_id
                ) as c ON messages.ts = c.join_ts AND messages.user_id = c.parent_user_id
                WHERE channel_id = $1 AND ts > $2 AND ts > $3 AND messages.parent_user_id = ''
                ORDER BY ts ASC LIMIT $4
                "#,
                channel_id,
                since_timestamp,
                timestamp,
                *msgs_per_page as i64
            )
            .fetch_all(&self.tummy_conn_pool)
            .await?
        } else {
            query_as!(
                DBParentMessage,
                "
                SELECT messages.*, users.*, c.cnt
                FROM messages
                INNER JOIN users ON users.id = messages.user_id
                LEFT JOIN (
                    SELECT COUNT(*) as cnt, thread_ts as join_ts, parent_user_id
                    FROM messages
                    WHERE channel_id = $1
                    GROUP BY join_ts, parent_user_id
                ) as c ON messages.ts = c.join_ts AND messages.user_id = c.parent_user_id
                WHERE channel_id = $1 AND ts > $2 AND messages.parent_user_id = ''
                ORDER BY ts ASC LIMIT $3
	            ",
                channel_id,
                since_timestamp,
                *msgs_per_page as i64
            )
            .fetch_all(&self.tummy_conn_pool)
            .await?
        };
        Ok(fetched_messages
            .into_iter()
            .map(models::Message::from)
            .collect())
    }

    pub async fn get_user_info(&self, user_id: &str) -> Result<User, sqlx::Error> {
        let user = query_as!(DBUser, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&self.tummy_conn_pool)
            .await?;
        Ok(user.into())
    }
}
