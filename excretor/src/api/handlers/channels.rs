//! Channel-related API handlers.
//! Provides endpoints for listing all channels and loading details for a specific channel.

use crate::api::errors::AppError;
use crate::api::routes::RouterState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::{Response, Json}};
use crate::api::models::{ChannelsResponse, ChannelDetailsResponse};

/// Fetches all available channels.
///
/// # Parameters
/// - `state`: Shared application state.
///
/// # Returns
/// On success, returns a JSON response with a list of channels and HTTP 200 OK.
/// On failure, returns an application error.
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

/// Loads details for a specific channel, including recent messages.
///
/// # Parameters
/// - `state`: Shared application state.
/// - `channel`: The channel ID as a path parameter.
///
/// # Returns
/// On success, returns a JSON response with channel details, last message timestamp,
/// messages, and channel ID, with HTTP 200 OK.
/// On failure, returns an application error.
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