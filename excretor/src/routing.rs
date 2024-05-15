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
        .route("/styles.css", get(handlers::styles))
        .route("/fallback-avatar", get(handlers::fallback_avatar))
        .route("/avatar.png", get(handlers::fallback_avatar_png))
        .with_state(RouterState { tummy })
}

mod handlers {
    use super::RouterState;
    use crate::templates;
    use askama::Template;
    use axum::extract::{Path, Query, State};
    use axum::response::IntoResponse;
    use axum::{
        body::Body,
        http::StatusCode,
        response::{Html, Response},
    };
    use serde::Deserialize;
    use sqlx::types::chrono;
    use tokio_util::io::ReaderStream;

    #[derive(Deserialize)]
    pub struct Pagination {
        last_msg_timestamp: Option<String>,
        per_page: u32,
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

    pub(super) async fn load_channel(
        State(state): State<RouterState>,
        Path(channel): Path<String>,
    ) -> (StatusCode, Response) {
        match state
            .tummy
            .get_channel_info(&channel)
            .await
            .map_err(internal_error)
        {
            Err(err) => err,
            Ok(channel) => (
                StatusCode::OK,
                Html(templates::ChannelTemplate { channel }.render().unwrap()).into_response(),
            ),
        }
    }

    pub(super) async fn get_messages(
        State(state): State<RouterState>,
        Path(channel_name): Path<String>,
        pagination: Query<Pagination>,
    ) -> (StatusCode, Response) {
        match state
            .tummy
            .fetch_msg_page(
                &channel_name,
                &pagination.last_msg_timestamp.as_ref().map(|ts_string| {
                    chrono::NaiveDateTime::parse_from_str(ts_string, "%Y-%m-%d %X%.f").unwrap()
                }),
                &pagination.per_page,
            )
            .await
            .map_err(internal_error)
        {
            Err(err) => err,
            Ok(messages) => {
                let new_last_msg_timestamp = messages
                    .last()
                    .map(|msg| msg.message.timestamp)
                    .unwrap_or(chrono::NaiveDateTime::UNIX_EPOCH);

                (
                    StatusCode::OK,
                    Html(
                        templates::ChannelPageTemplate {
                            messages,
                            last_msg_timestamp: new_last_msg_timestamp.to_string(),
                            channel_name,
                        }
                        .render()
                        .unwrap(),
                    )
                    .into_response(),
                )
            }
        }
    }
    pub(super) async fn styles() -> impl IntoResponse {
        let file = match tokio::fs::File::open("./templates/styles.css").await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };
        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);

        Ok(body)
    }
    pub(super) async fn fallback_avatar_png() -> impl IntoResponse {
        let file = match tokio::fs::File::open("./templates/avatar.png").await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };
        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);

        Ok(body)
    }
    pub(super) async fn fallback_avatar() -> (StatusCode, Response) {
        (
            StatusCode::OK,
            Html(templates::FallbackAvatarTemplate.render().unwrap()).into_response(),
        )
    }
}
