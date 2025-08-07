use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, Response};
use tokio_util::io::ReaderStream;
use crate::api::routes::RouterState;
use crate::api::errors::AppError;
use axum::response::IntoResponse;


pub async fn fallback_avatar() -> Result<(StatusCode, Response), AppError> {
    todo!();
    // Ok((
    //     StatusCode::OK,
    //     Html(templates::FallbackAvatarTemplate.render()?).into_response(),
    // ))
}

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

pub async fn serve_react_app() -> Result<(StatusCode, Response), AppError> {
    Ok((StatusCode::OK, "Hello app!".into_response()))
}