use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use serde::Deserialize;

mod env;
mod models;
mod templates;
mod tummy;

use models::{Channel, Message, User};
use tracing::info;

use tracing_subscriber::prelude::*;
use tummy::Tummy;

#[derive(Clone)]
struct RouterState {
    tummy: Tummy,
}

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
        .route("/", get(root))
        .route("/channels/:channel", get(load_channel))
        .route("/messages/:channel", get(get_messages))
        .with_state(RouterState { tummy });

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

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, Response)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(
            templates::ErrTemplate {
                err_string: err.to_string(),
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    )
}

// basic handler that responds with a static string
async fn root(State(state): State<RouterState>) -> (StatusCode, Response) {
    match state.tummy.get_channels().await.map_err(internal_error) {
        Err(err) => err,
        Ok(channels) => (
            StatusCode::OK,
            Html(templates::IndexTemplate { channels }.render().unwrap()).into_response(),
        ),
    }
}

async fn load_channel(Path(channel): Path<String>) -> (StatusCode, Response) {
    (
        StatusCode::OK,
        Html(
            templates::ChannelTemplate {
                channel: Channel { name: channel, purpose: "Nothing rn".into(), topic: "Hey if you are the frontend developer, make sure to ask someone to return the actual data.".into() },
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
            channel_name: "idk yet".into(),
            user_id: "A user.".into(),
            ts: "0".into(),
            thread_ts: "0".into(),
            parent_user_id: "Another user".into(),
            user: User {
                real_name: "Lmao the frontend devs are going to have a field day with this.".into(),
                display_name: "Seriously though, the db has not yet been integrated here, so please bare with me.".into(),
                email: "backend_developer@hell.com".into(),
                deleted: false,
                is_bot: false,
                id: i as i32,
                name: format!("User {}", i + 1),
                image_url: String::new(),
            },
        })
    }

    (
        StatusCode::OK,
        Html(
            templates::ChannelPageTemplate {
                messages,
                page: pagination.page,
                channel: Channel { name: channel, topic: "Dear frontend developers. I am sorry you have to go through this but that's just how it is.".into(), purpose: "You are either a frontend developer cursing me right now or a backend developer. Neither case is good but either is necessary at the moment.".into() },
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    )
}
