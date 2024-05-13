use askama::Template;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

mod env;
mod models;
mod templates;
mod tummy;

use models::{Channel, Message, User};
use tracing::info;

use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    // Read environment variables
    let env_vars = env::EnvVars::init();

    let stdout_log = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(stdout_log).init();

    let tummy_conn_pool = tummy::get_tummy_conn_pool(&env_vars).await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/channels/:channel", get(load_channel))
        .route("/messages/:channel", get(get_messages))
        .with_state(tummy_conn_pool);

    info!("Starting excretor on port {}.", env_vars.excretor_port);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("Excretor listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

// basic handler that responds with a static string
async fn root() -> (StatusCode, Response) {
    let mut channels: Vec<Channel> = vec![];
    for i in 0..10 {
        channels.push(Channel {
            name: format!("Channel-{}", i),
        })
    }

    (
        StatusCode::OK,
        Html(templates::IndexTemplate { channels }.render().unwrap()).into_response(),
    )
}

async fn load_channel(Path(channel): Path<String>) -> (StatusCode, Response) {
    (
        StatusCode::OK,
        Html(
            templates::ChannelTemplate {
                channel: Channel { name: channel },
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    )
}

async fn get_messages(
    Path(channel): Path<String>,
    pagination: Query<Pagination>,
) -> (StatusCode, Response) {
    // Generate test messages
    let mut messages: Vec<Message> = vec![];
    for i in (pagination.page - 1) * pagination.per_page..pagination.page * pagination.per_page {
        messages.push(Message {
            id: i as i32,
            text: format!("Test message {}", i + 1),
            user: User {
                id: i as i32,
                name: format!("User {}", i + 1),
                avatar_url: String::new(),
            },
        })
    }

    (
        StatusCode::OK,
        Html(
            templates::ChannelPageTemplate {
                messages,
                page: pagination.page,
                channel: Channel { name: channel },
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    )
}
