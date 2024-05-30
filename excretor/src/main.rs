use clap::Parser;

mod dbmodels;
mod env;
mod models;
mod routing;
mod templates;
mod tummy;

use tracing::info;

use tracing_subscriber::prelude::*;
use tummy::Tummy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read environment variables
    let env_vars = env::EnvVars::parse().process()?;

    let stdout_log = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(stdout_log).init();

    let tummy = Tummy::init(&env_vars).await;
    let app = routing::get_excretor_router(tummy, env_vars.clone());

    info!("Starting excretor on port {}.", env_vars.excretor_port);
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env_vars.excretor_port)).await?;
    tracing::debug!("Excretor listening on {}.", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
