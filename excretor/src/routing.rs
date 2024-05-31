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

    pub(super) struct AppError(color_eyre::eyre::Error);

    impl IntoResponse for AppError {
        fn into_response(self) -> axum::response::Response {
            tracing::error!("An error occured: {}", self.0);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Something went wrong. Please try again later"),
            )
                .into_response()
        }
    }

    #[derive(Deserialize)]
    pub struct ReplyRequest {
        pub channel_id: String,
        pub ts: String,
        pub user_id: String,
    }

    /// Utility function for mapping any error into a `500 Internal Server Error`
    /// response.
    impl<E> From<E> for AppError
    where
        E: Into<color_eyre::eyre::Error>,
    {
        fn from(err: E) -> Self {
            Self(err.into())
        }
    }

    #[derive(Deserialize)]
    pub struct Pagination {
        last_msg_timestamp: Option<String>,
        per_page: u32,
    }

    // basic handler that responds with a static string
    pub(super) async fn root(
        State(state): State<RouterState>,
    ) -> Result<(StatusCode, Response), AppError> {
        let channels = state.tummy.get_all_channels().await?;

        Ok((
            StatusCode::OK,
            Html(templates::IndexTemplate { title: state.env_vars.title, description: state.env_vars.description, channels }.render()?).into_response(),
        ))
    }

    pub(super) async fn load_channel(
        State(state): State<RouterState>,
        Path(channel): Path<String>,
    ) -> Result<(StatusCode, Response), AppError> {
        let channel = state.tummy.get_channel_info(&channel).await?;

        Ok((
            StatusCode::OK,
            Html(templates::ChannelTemplate { channel }.render()?).into_response(),
        ))
    }

    pub(super) async fn get_messages(
        State(state): State<RouterState>,
        Path(channel_id): Path<String>,
        pagination: Query<Pagination>,
    ) -> Result<(StatusCode, Response), AppError> {
        let messages = state
            .tummy
            .fetch_msg_page(
                &channel_id,
                &pagination
                    .last_msg_timestamp
                    .as_ref()
                    .map(|ts| chrono::NaiveDateTime::from_pg_ts(ts)),
                &pagination.per_page,
            )
            .await?;

        let new_last_msg_timestamp = messages
            .last()
            .map(|message| message.timestamp)
            .unwrap_or(chrono::NaiveDateTime::UNIX_EPOCH);

        Ok((
            StatusCode::OK,
            Html(
                templates::ChannelPageTemplate {
                    messages,
                    last_msg_timestamp: new_last_msg_timestamp.to_string(),
                    channel_id,
                }
                .render()?,
            )
            .into_response(),
        ))
    }

    pub(super) async fn get_replies(
        State(state): State<RouterState>,
        message_data: Query<ReplyRequest>,
    ) -> Result<(StatusCode, Response), AppError> {
        let messages = state
            .tummy
            .fetch_replies(
                &message_data.ts,
                &message_data.channel_id,
                &message_data.user_id,
            )
            .await?;

        Ok((
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
        ))
    }

    pub(super) async fn fallback_avatar() -> Result<(StatusCode, Response), AppError> {
        Ok((
            StatusCode::OK,
            Html(templates::FallbackAvatarTemplate.render()?).into_response(),
        ))
    }

    pub(super) async fn assets(
        State(state): State<RouterState>,
        Path(filepath): Path<String>,
    ) -> Result<(StatusCode, Response), AppError> {
        let final_file_path = state
            .env_vars
            .static_assets_dir
            .join(&filepath)
            .canonicalize()?;

        if final_file_path.starts_with(state.env_vars.static_assets_dir) {
            let file = tokio::fs::File::open(final_file_path).await?;

            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            Ok((StatusCode::OK, body.into_response()))
        } else {
            tracing::warn!(
                "A mortal requested to access forbidden file `{}`.",
                filepath
            );

            Ok((
            	StatusCode::FORBIDDEN,
             	Body::from(String::from("Mortals are forbidden from accessing the requested file. This sin will be reported.")).into_response()
            ))
        }
    }
}
