use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{self, NaiveDateTime},
    PgPool,
};
use std::time::Duration;

use crate::{
    dbmodels::{DBChannel, DBMessageAndUser}, env::EnvVars, models::{self, Channel, Message, User}
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

    pub async fn get_all_channels(&self) -> Result<Vec<Channel>, sqlx::Error> {
        let db_channels = sqlx::query_as::<_, DBChannel>(queries::GET_ALL_CHANNELS)
            .fetch_all(&self.tummy_conn_pool)
            .await?;
        Ok(db_channels.iter().map(models::Channel::from_db_channel).collect())
    }

    pub async fn get_channel_info(&self, channel_name: &str) -> Result<Channel, sqlx::Error> {
        let channel = sqlx::query_as::<_, DBChannel>(queries::GET_CHANNEL_FROM_NAME)
            .bind(channel_name)
            .fetch_one(&self.tummy_conn_pool)
            .await?;
        Ok(models::Channel::from_db_channel(&channel))
    }

    pub async fn fetch_msg_page(
        &self,
        channel_name: &str,
        last_msg_timestamp: &Option<chrono::NaiveDateTime>,
        msgs_per_page: &u32,
    ) -> Result<Vec<(Message, User)>, sqlx::Error> {
        let fetched_messages = if let Some(timestamp) = last_msg_timestamp {
            sqlx::query_as::<_, DBMessageAndUser>(queries::GET_MSG_USER_JOIN_BEFORE_TS)
                .bind(channel_name)
                .bind(timestamp)
                .bind(i64::from(*msgs_per_page))
                .fetch_all(&self.tummy_conn_pool)
                .await
        } else {
            sqlx::query_as::<_, DBMessageAndUser>(queries::GET_MSG_USER_JOIN)
                .bind(channel_name)
                .bind(i64::from(*msgs_per_page))
                .fetch_all(&self.tummy_conn_pool)
                .await
        }?;
        
        let messages_and_users = fetched_messages
            .iter()
            .map(|e| (models::Message::from_db_message(&e.message), models::User::from_db_user(&e.user)))
            .collect();
        Ok(messages_and_users)
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
