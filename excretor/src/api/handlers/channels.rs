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
/// - `channel`: The channel id as a path parameter.
///
/// # Returns
/// On success, returns a JSON response with channel details, last message timestamp,
/// messages, and channel ID, with HTTP 200 OK.
/// On failure, returns an application error.
pub async fn load_channel(
    State(state): State<RouterState>,
    Path(channel_id): Path<String>,
) -> Result<(StatusCode, Response), AppError> {
    let channel = state.tummy.get_channel_info(&channel_id).await?;
    let messages = state
        .tummy
        .fetch_msg_page(&channel.id, &None, &10)
        .await?;
    let channel_id = channel.id.clone();
    Ok((
        StatusCode::OK,
        Json(
            ChannelDetailsResponse{
                channel,
                before_msg_timestamp: if let Some(last_msg) = messages.first() {
                    Some(last_msg.timestamp.format("%Y-%m-%dT%H:%M:%S%.f").to_string())
                } else {
                    None
                },
                messages,
                channel_id,
            }
        ).into_response(),
    ))
}