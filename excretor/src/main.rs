use clap::Parser;

mod dbmodels;
mod env;
mod models;
mod templates;
mod tummy;
mod db;
mod api;
mod types;

use tracing::info;

use tracing_subscriber::prelude::*;

use db::tummy::Tummy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read environment variables
    let env_vars = env::EnvVars::parse().process()?;

    let stdout_log = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(stdout_log).init();

    let db_connection = Tummy::init(&env_vars).await;
    let app = api::routes::get_excretor_router(db_connection, env_vars.clone());

    info!("Starting excretor on port {}.", env_vars.excretor_port);
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env_vars.excretor_port)).await?;
    tracing::debug!("Excretor listening on {}.", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
