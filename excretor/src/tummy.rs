use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::{env::EnvVars, models::Channel};

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

    pub async fn get_channels(&self) -> Result<Vec<Channel>, sqlx::Error> {
        sqlx::query_as::<_, Channel>("select * from channels")
            .fetch_all(&self.tummy_conn_pool)
            .await
    }
}
