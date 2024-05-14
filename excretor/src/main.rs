use axum::{routing::get, Router};
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

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(routing::handlers::root))
        .route("/channels/:channel", get(routing::handlers::load_channel))
        .route("/messages/:channel", get(routing::handlers::get_messages))
        .with_state(routing::RouterState { tummy });

    info!("Starting excretor on port {}.", env_vars.excretor_port);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("Excretor listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
