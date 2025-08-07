use crate::api::errors::AppError;
use crate::api::routes::RouterState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::{Response, Json}};
use crate::api::models::{ChannelsResponse, ChannelDetailsResponse};

pub async fn get_channels(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Response), AppError> {
    let channels = state.tummy.get_all_channels().await?;
    Ok((
        StatusCode::OK,
        Json(
            ChannelsResponse {
                channels,
            }
        ).into_response(),
    ))
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
    Ok((
        StatusCode::OK,
        Json(
            ChannelDetailsResponse{
                channel,
                last_msg_timestamp: if let Some(last_msg) = messages.last() {
                    last_msg.timestamp.to_string()
                } else {
                    sqlx::types::chrono::DateTime::UNIX_EPOCH.naive_utc().to_string()
                },
                messages,
                channel_id,
            }
        ).into_response(),
    ))
}