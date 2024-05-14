use crate::tummy::Tummy;
use axum::{routing::get, Router};

#[derive(Clone)]
struct RouterState {
    pub tummy: Tummy,
}

pub fn get_excretor_router(tummy: Tummy) -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(handlers::root))
        .route("/channels/:channel", get(handlers::load_channel))
        .route("/messages/:channel", get(handlers::get_messages))
        .with_state(RouterState { tummy })
}

mod handlers {
    use super::RouterState;
    use crate::models::{Channel, Message, User};
    use crate::templates;
    use askama::Template;
    use axum::extract::{Path, Query, State};
    use axum::response::IntoResponse;
    use axum::{
        http::StatusCode,
        response::{Html, Response},
    };
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Pagination {
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
    pub(super) async fn root(State(state): State<RouterState>) -> (StatusCode, Response) {
        match state.tummy.get_all_channels().await.map_err(internal_error) {
            Err(err) => err,
            Ok(channels) => (
                StatusCode::OK,
                Html(templates::IndexTemplate { channels }.render().unwrap()).into_response(),
            ),
        }
    }

    pub(super) async fn load_channel(Path(channel): Path<String>) -> (StatusCode, Response) {
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

    pub(super) async fn get_messages(
        Path(channel): Path<String>,
        pagination: Query<Pagination>,
    ) -> (StatusCode, Response) {
        // Generate test messages
        let mut messages: Vec<(Message, User)> = vec![];
        for i in (pagination.page - 1) * pagination.per_page..pagination.page * pagination.per_page
        {
            messages.push(
                (Message {
                    id: i as i32,
                    text: format!("Test message {}", i + 1),
                    channel_name: "idk yet".into(),
                    user_id: "A user.".into(),
                    timestamp: "0".into(),
                    thread_timestamp: "0".into(),
                    parent_user_id: "Another user".into(),
                },  User {
                    real_name: "Lmao the frontend devs are going to have a field day with this.".into(),
                    display_name: "Seriously though, the db has not yet been integrated here, so please bare with me.".into(),
                    email: "backend_developer@hell.com".into(),
                    deleted: false,
                    is_bot: false,
                    id: i as i32,
                    name: format!("User {}", i + 1),
                    image_url: String::new(),
                })
            );
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
}
