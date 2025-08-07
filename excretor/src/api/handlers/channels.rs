use crate::api::errors::AppError;
use crate::api::routes::RouterState;
use crate::templates;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::{Html, Response}};


pub async fn get_channels(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Response), AppError> {
    let channels = state.tummy.get_all_channels().await?;
    todo!();
    // Ok((
    //     StatusCode::OK,
    //     Html(
    //         templates::IndexTemplate {
    //             title: state.env_vars.title,
    //             description: state.env_vars.description,
    //             channels,
    //         }
    //             .render()?,
    //     )
    //         .into_response(),
    // ))
}

pub async fn load_channel(
    State(state): State<RouterState>,
    Path(channel): Path<String>,
) -> Result<(StatusCode, Response), AppError> {
    let channel = state.tummy.get_channel_info(&channel).await?;
    let messages = state
        .tummy
        .fetch_msg_page(&channel.id, &None, &10, &sqlx::types::chrono::DateTime::UNIX_EPOCH.naive_utc())
        .await?;

    let channel_id = channel.id.clone();
    todo!();
    // Ok((
    //     StatusCode::OK,
    //     Html(templates::ChannelTemplate {
    //         channel,
    //         last_msg_timestamp: if let Some(last_msg) = messages.last() {
    //             last_msg.timestamp.to_string()
    //         } else {
    //             sqlx::types::chrono::DateTime::UNIX_EPOCH.naive_utc().to_string()
    //         },
    //         messages,
    //         channel_id,
    //     }.render()?).into_response(),
    // ))
}