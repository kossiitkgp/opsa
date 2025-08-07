//! Miscellaneous API handlers.
//! Provides endpoints for serving static assets and the React frontend application.

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use tokio_util::io::ReaderStream;
use crate::api::routes::RouterState;
use crate::api::errors::AppError;
use axum::response::IntoResponse;

/// Serves static asset files from the configured directory.
///
/// # Parameters
/// - `state`: Shared application state containing environment variables.
/// - `filepath`: Path to the requested static file, relative to the assets directory.
///
/// # Returns
/// On success, returns the file contents as a streamed response with HTTP 200 OK.
/// If the file is outside the allowed directory, returns HTTP 403 Forbidden.
/// On failure, returns an application error.
pub async fn assets(
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

/// Serves the React frontend application.
///
/// # Returns
/// Returns a placeholder response for the React app with HTTP 200 OK.
pub async fn serve_react_app() -> Result<(StatusCode, Response), AppError> {
    Ok((StatusCode::OK, "Hello app!".into_response()))
}