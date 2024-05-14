use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::{
    env::EnvVars,
    models::{Channel, MessageAndUser},
};

#[derive(Clone)]
pub struct Tummy {
    tummy_conn_pool: PgPool,
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
        sqlx::query_as::<_, Channel>("select * from channels")
            .fetch_all(&self.tummy_conn_pool)
            .await
    }

    pub async fn get_channel_info(&self, channel_name: &str) -> Result<Channel, sqlx::Error> {
        sqlx::query_as::<_, Channel>("select * from channels where name = $1")
            .bind(channel_name)
            .fetch_one(&self.tummy_conn_pool)
            .await
    }

    pub async fn fetch_msg_page(
        &self,
        channel_name: &str,
        last_msg_timestamp: &str,
        msgs_per_page: &usize,
    ) -> Result<Vec<MessageAndUser>, sqlx::Error> {
        sqlx::query_as::<_, MessageAndUser>(
            "SELECT messages.*, uesrs.* FROM messages WHERE ts < $1 AND channel_name = $2 ORDER BY ts DESC LIMIT $3 INNER JOIN users ON users.id = messages.uesr_id",
        )
        .bind(last_msg_timestamp)
        .bind(channel_name)
        .bind(msgs_per_page.to_string())
        .fetch_all(&self.tummy_conn_pool)
        .await
    }
}
