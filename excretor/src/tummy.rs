use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{self, NaiveDateTime},
    PgPool,
};
use std::time::Duration;

use crate::{
    env::EnvVars,
    models::{Channel, MessageAndUser},
};

#[derive(Clone)]
pub struct Tummy {
    tummy_conn_pool: PgPool,
}

pub fn get_formatted_timestamp(timestamp: &NaiveDateTime) -> String {
    timestamp.format("%d %b %Y @ %I:%M %p").to_string()
}
pub fn str_to_datetime(ts_string: &String) -> NaiveDateTime {
    chrono::NaiveDateTime::parse_from_str(ts_string, "%Y-%m-%d %X%.f").unwrap()
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

    pub async fn get_all_channels(&self) -> Result<Vec<Channel>, sqlx::Error> {
        sqlx::query_as::<_, Channel>(queries::GET_ALL_CHANNELS)
            .fetch_all(&self.tummy_conn_pool)
            .await
    }

    pub async fn get_channel_info(&self, channel_name: &str) -> Result<Channel, sqlx::Error> {
        sqlx::query_as::<_, Channel>(queries::GET_CHANNEL_FROM_NAME)
            .bind(channel_name)
            .fetch_one(&self.tummy_conn_pool)
            .await
    }

    pub async fn fetch_msg_page(
        &self,
        channel_name: &str,
        last_msg_timestamp: &Option<chrono::NaiveDateTime>,
        msgs_per_page: &u32,
    ) -> Result<Vec<MessageAndUser>, sqlx::Error> {
        let mut fetched_messages = if let Some(timestamp) = last_msg_timestamp {
            sqlx::query_as::<_, MessageAndUser>(queries::GET_MSG_USER_JOIN_BEFORE_TS)
                .bind(channel_name)
                .bind(timestamp)
                .bind(i64::from(*msgs_per_page))
                .fetch_all(&self.tummy_conn_pool)
                .await
        } else {
            sqlx::query_as::<_, MessageAndUser>(queries::GET_MSG_USER_JOIN)
                .bind(channel_name)
                .bind(i64::from(*msgs_per_page))
                .fetch_all(&self.tummy_conn_pool)
                .await
        }?;

        fetched_messages.iter_mut().for_each(|msg| {
            msg.message.formatted_timestamp = get_formatted_timestamp(&msg.message.timestamp);

            if msg.user.image_url.is_empty() {
                msg.user.image_url = "/assets/avatar.png".into();
            }
        });

        Ok(fetched_messages)
    }
}

mod queries {
    pub const GET_ALL_CHANNELS: &str = "SELECT * FROM channels";
    pub const GET_CHANNEL_FROM_NAME: &str = "SELECT * FROM channels WHERE name = $1";
    pub const GET_MSG_USER_JOIN_BEFORE_TS: &str = "
		SELECT messages.*, users.*
		FROM messsages
		INNER JOIN users ON users.id = messages.user_id
		WHERE channel_name $1 AND ts < $2
		ORDER BY ts DESC LIMIT $3
	";
    pub const GET_MSG_USER_JOIN: &str = "
		SELECT messages.*, users.*
		FROM messages
		INNER JOIN users ON users.id = messages.user_id
		WHERE channel_name = $1
		ORDER BY ts DESC LIMIT $2
	";
}
