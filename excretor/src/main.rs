use clap::Parser;

mod env;
mod models;
mod routing;
mod templates;
mod tummy;

use tracing::info;

use tracing_subscriber::prelude::*;
use tummy::Tummy;

#[tokio::main]
async fn main() {
    // Read environment variables
    let env_vars = env::EnvVars::parse();

    let stdout_log = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(stdout_log).init();

    let tummy = Tummy::init(&env_vars).await;
    let app = routing::get_excretor_router(tummy);

    info!("Starting excretor on port {}.", env_vars.excretor_port);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("Excretor listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
