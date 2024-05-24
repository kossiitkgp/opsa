use crate::{env::EnvVars, tummy::Tummy};
use axum::{routing::get, Router};

#[derive(Clone)]
struct RouterState {
    pub tummy: Tummy,
    pub env_vars: EnvVars,
}

pub fn get_excretor_router(tummy: Tummy, env_vars: EnvVars) -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(handlers::root))
        .route("/channels/:channel_name", get(handlers::load_channel))
        .route("/messages/:channel_id", get(handlers::get_messages))
        .route("/fallback-avatar", get(handlers::fallback_avatar))
        .route("/assets/*file", get(handlers::assets))
        .route("/replies", get(handlers::get_replies))
        .with_state(RouterState { tummy, env_vars })
}

mod handlers {
    use super::RouterState;
    use crate::templates;
    use crate::tummy::SlackDateTime;
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

    #[derive(Deserialize)]
    pub struct ReplyRequest {
        pub channel_id: String,
        pub ts: String,
        pub user_id: String,
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
        Path(channel_name): Path<String>,
    ) -> (StatusCode, Response) {
        match state
            .tummy
            .get_channel_info(&channel_name)
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
        Path(channel_id): Path<String>,
        pagination: Query<Pagination>,
    ) -> (StatusCode, Response) {
        match state
            .tummy
            .fetch_msg_page(
                &channel_id,
                &pagination
                    .last_msg_timestamp
                    .as_ref()
                    .map(|ts| chrono::NaiveDateTime::from_pg_ts(ts)),
                &pagination.per_page,
            )
            .await
            .map_err(internal_error)
        {
            Err(err) => err,
            Ok(messages) => {
                let new_last_msg_timestamp = messages
                    .last()
                    .map(|(message, _user)| message.timestamp)
                    .unwrap_or(chrono::NaiveDateTime::UNIX_EPOCH);
                (
                    StatusCode::OK,
                    Html(
                        templates::ChannelPageTemplate {
                            messages,
                            last_msg_timestamp: new_last_msg_timestamp.to_string(),
                            channel_id,
                        }
                        .render()
                        .unwrap(),
                    )
                    .into_response(),
                )
            }
        }
    }

    pub(super) async fn get_replies(
        State(state): State<RouterState>,
        message_data: Query<ReplyRequest>,
    ) -> (StatusCode, Response) {
        match state
            .tummy
            .fetch_replies(
                &message_data.ts,
                &message_data.channel_id,
                &message_data.user_id,
            )
            .await
            .map_err(internal_error)
        {
            Err(err) => err,
            Ok(messages) => {
                (
                    StatusCode::OK,
                    Html(
                        templates::ThreadTemplate {
                            messages,
                            parent_ts: message_data.ts.clone(),
                            channel_id: message_data.channel_id.clone(),
                            parent_user_id: message_data.user_id.clone(),
                        }
                        .render()
                        .unwrap(),
                    )
                    .into_response(),
                )
            }
        }
    }

    pub(super) async fn fallback_avatar() -> (StatusCode, Response) {
        (
            StatusCode::OK,
            Html(templates::FallbackAvatarTemplate.render().unwrap()).into_response(),
        )
    }

    pub(super) async fn assets(
        State(state): State<RouterState>,
        Path(filepath): Path<String>,
    ) -> (StatusCode, Response) {
        // TODO: Remove the unwrap once axum error handling is oxidized. (Issue #27)
        let final_file_path = state
            .env_vars
            .static_assets_dir
            .join(&filepath)
            .canonicalize()
            .unwrap();

        if final_file_path.starts_with(state.env_vars.static_assets_dir) {
            match tokio::fs::File::open(final_file_path).await {
                Ok(file) => {
                    let stream = ReaderStream::new(file);
                    let body = Body::from_stream(stream);

                    (StatusCode::OK, body.into_response())
                }
                Err(err) => {
                    tracing::warn!("Error while serving asset file `{}`: {}", filepath, err);

                    (
                        StatusCode::NOT_FOUND,
                        Body::from(String::from("The requested file was not found."))
                            .into_response(),
                    )
                }
            }
        } else {
            tracing::warn!(
                "A mortal requested to access forbidden file `{}`.",
                filepath
            );

            (
            	StatusCode::FORBIDDEN,
             	Body::from(String::from("Mortals are forbidden from accessing the requested file. This sin will be reported.")).into_response()
            )
        }
    }
}
