use askama::Template;
use axum::{
    extract::{Path, Query},
    http::{response, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

mod templates;
mod models;

use models::{Channel, Message, User};

static PER_PAGE:usize = 50;

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
#[derive(Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
    pane: Option<String>
}

// basic handler that responds with a static string
async fn root() -> (StatusCode, Response) {
    let mut channels:Vec<Channel> = vec![];
    for i in 0..10 {
        channels.push(Channel{
            name:format!("Channel-{}", i)
        })
    }

    (StatusCode::OK, Html(templates::IndexTemplate{
        channels:channels,
        per_page: PER_PAGE
    }.render().unwrap()).into_response())
}

async fn load_channel(Path(channel): Path<String>) -> (StatusCode, Response) {
    let mut messages: Vec<Message> = vec![];
    for i in 0..PER_PAGE {
        messages.push(Message{
            id: i as i32,
            text: format!("Test message {}", i+1),
            user: User{
                id: i as i32,
                name: format!("User {}", i+1),
                avatar_url: format!("")
            }
        })
    }
    
    (StatusCode::OK, Html(templates::ChannelTemplate{
        channel: Channel{name: channel},
        per_page: PER_PAGE,
        messages
    }.render().unwrap()).into_response())
}

async fn get_messages(Path(channel): Path<String>, pagination: Query<Pagination>) -> (StatusCode, Response) {

    if pagination.page == 0 {
        return (StatusCode::BAD_REQUEST, Html("Invalid page number").into_response());
    }

    // Generate test messages
    let mut messages: Vec<Message> = vec![];
    for i in (pagination.page-1)*pagination.per_page..pagination.page*pagination.per_page {
        messages.push(Message{
            id: i as i32,
            text: format!("Test message {}", i+1),
            user: User{
                id: i as i32,
                name: format!("User {}", i+1),
                avatar_url: format!("")
            }
        })
    }

    let mut response = String::new();
    
    if let Some(pane) = &pagination.pane {
        if pane == "forward" {
            response.push_str(&format!("<div hx-swap-oob=\"delete\" id=\"forward-event\"></div>"));
            response.push_str(&format!("<div hx-swap-oob=\"true\" id=\"backward-event\" hx-get=\"/messages/{}?page={}&per_page={}&pane=backward\" hx-target=\"#page-{}\" hx-swap=\"beforebegin\" hx-trigger=\"revealed\">Loading...</div>", channel.clone(), pagination.page-3, pagination.per_page, pagination.page-2));
            response.push_str(&format!("<div hx-swap-oob=\"delete\" id=\"page-{}\"></div>", pagination.page-3));
        } else if pane == "backward" {
            response.push_str(&format!("<div hx-swap-oob=\"delete\" id=\"backward-event\"></div>"));
            response.push_str(&format!("<div hx-swap-oob=\"true\" id=\"forward-event\" hx-get=\"/messages/{}?page={}&per_page={}&pane=forward\" hx-target=\"#page-{}\" hx-swap=\"afterend\" hx-trigger=\"revealed\">Loading...</div>", channel.clone(), pagination.page+3, pagination.per_page, pagination.page+2));
            response.push_str(&format!("<div hx-swap-oob=\"delete\" id=\"page-{}\"></div>", pagination.page+3));
            response.push_str(&format!("<div id=\"backward-event\" hx-get=\"/messages/{}?page={}&per_page={}&pane=backward\" hx-target=\"#page-{}\" hx-swap=\"beforebegin\" hx-trigger=\"revealed\">Loading...</div>", channel, pagination.page-1, pagination.per_page, pagination.page));
        }
    }

    response.push_str(&templates::ChannelPageTemplate{
        messages:messages, 
        page:pagination.page,
        channel: Channel{name: channel.clone()},
        per_page: PER_PAGE
    }.render().unwrap());

    if let Some(pane) = &pagination.pane {
        if pane == "forward" {
            response.push_str(&format!("<div id=\"forward-event\" hx-get=\"/messages/{}?page={}&per_page={}&pane=forward\" hx-target=\"#page-{}\" hx-swap=\"afterend\" hx-trigger=\"revealed\">Loading...</div>", channel.clone(), pagination.page+1, pagination.per_page, pagination.page));
        }
    }


    (StatusCode::OK, Html(response).into_response())
}