use askama::Template;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

mod models;
mod templates;

use models::{Channel, Message, User};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/channels/:channel", get(load_channel))
        .route("/messages/:channel", get(get_messages));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
