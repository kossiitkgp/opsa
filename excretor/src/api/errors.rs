use axum::http::StatusCode;
use axum::response::{IntoResponse};


pub(in crate::api) struct AppError(color_eyre::eyre::Error);

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

impl<E> From<E> for AppError
where
    E: Into<color_eyre::eyre::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}