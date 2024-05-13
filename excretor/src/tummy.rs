use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::env::EnvVars;

pub async fn get_tummy_conn_pool(env_vars: &EnvVars) -> PgPool {
    let tummy_conn_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        env_vars.tummy_username,
        env_vars.tummy_password,
        env_vars.tummy_host,
        env_vars.tummy_port,
        env_vars.tummy_db
    );

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&tummy_conn_string)
        .await
        .expect("Could not connect to tummy.")
}
