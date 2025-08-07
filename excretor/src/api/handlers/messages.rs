//! Message-related API handlers.
//! Provides endpoints for searching messages, fetching messages for a channel,
//! and retrieving replies in a thread.

use crate::db::tummy::SlackDateTime;
use crate::api::errors::AppError;
use crate::api::routes::RouterState;
use axum::response::IntoResponse;
use axum::extract::{Form, Path, Query, State};
use axum::{http::StatusCode, response::Response, Json};
use serde::Deserialize;
use crate::api::models;

/// Request payload for fetching replies to a message.
#[derive(Deserialize)]
pub struct ReplyRequest {
    /// Channel ID where the message is posted.
    pub channel_id: String,
    /// Timestamp of the parent message.
    pub ts: String,
    /// User ID of the parent message author.
    pub user_id: String,
}

/// Form data for searching messages.
#[derive(Deserialize)]
pub struct SearchQuery {
    /// Search query string.
    query: String,
    /// Optional channel ID to filter search.
    channel_id: Option<String>,
    /// Optional user ID to filter search.
    user_id: Option<String>,
}

/// Query parameters for paginating messages.
#[derive(Deserialize)]
pub struct Pagination {
    /// Timestamp of the last message from the previous page.
    before_msg_timestamp: Option<String>,
    /// Number of messages per page.
    per_page: u32,
}

/// Query parameters for filtering messages by date.
#[derive(Deserialize)]
pub struct DateQuery {
    /// Optional ISO date string to fetch messages since this date.
    after: Option<String>,
    before: Option<String>,
}

/// Searches messages by text, channel, and user.
///
/// # Parameters
/// - `state`: Shared application state.
/// - `payload`: Form data containing the search query and optional filters.
///
/// # Returns
/// On success, returns a JSON response with matching messages and HTTP 200 OK.
/// On failure, returns an application error.
pub async fn search(
    State(state): State<RouterState>,
    Form(payload): Form<SearchQuery>,
) -> Result<(StatusCode, Response), AppError> {
    let messages = state
        .tummy
        .search_msg_text(
            &payload.query,
            payload.channel_id.as_deref(),
            payload.user_id.as_deref(),
            10,
            0.1
        )
        .await?;
    Ok((
        StatusCode::OK,
        Json(
            models::SearchResultsResponse {
                messages,
                query: payload.query,
            }
        ).into_response()
    ))
}

/// Fetches messages for a specific channel, with pagination and optional date filter.
///
/// # Parameters
/// - `state`: Shared application state.
/// - `channel_id`: The channel ID as a path parameter.
/// - `pagination`: Query parameters for pagination.
/// - `date_query`: Query parameters for date filtering.
///
/// # Returns
/// On success, returns a JSON response with messages, last message timestamp, and channel ID, with HTTP 200 OK.
/// On failure, returns an application error.
pub async fn get_messages(
    State(state): State<RouterState>,
    Path(channel_id): Path<String>,
    pagination: Query<Pagination>,
    _date_query: Query<DateQuery>,
) -> Result<(StatusCode, Response), AppError> {
    let messages = state
        .tummy
        .fetch_msg_page(
            &channel_id,
            &pagination
                .before_msg_timestamp
                .as_ref()
                .map(|ts| sqlx::types::chrono::NaiveDateTime::from_pg_ts(ts)),
            &pagination.per_page,
        )
        .await?;

    let oldest_message_timestamp = messages
        .first()
        .map(|message| message.timestamp)
        .unwrap_or(sqlx::types::chrono::DateTime::UNIX_EPOCH.naive_utc());
    Ok((
        StatusCode::OK,
        Json(
            models::MessagesResponse {
                messages,
                before_msg_timestamp: oldest_message_timestamp.format("%Y-%m-%dT%H:%M:%S%.f").to_string(),
                channel_id,
            }
        ).into_response(),
    ))
}

/// Fetches replies for a specific message (thread).
///
/// # Parameters
/// - `state`: Shared application state.
/// - `message_data`: Query parameters containing the parent message's channel ID, timestamp, and user ID.
///
/// # Returns
/// On success, returns a JSON response with thread messages, parent timestamp, channel ID, and parent user ID, with HTTP 200 OK.
/// On failure, returns an application error.
pub async fn get_replies(
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
        Json(
            models::ThreadResponse {
                messages,
                parent_ts: message_data.ts.clone(),
                channel_id: message_data.channel_id.clone(),
                parent_user_id: message_data.user_id.clone(),
            }
        ).into_response()
    ))
}