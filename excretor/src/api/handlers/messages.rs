use crate::db::tummy::SlackDateTime;
use crate::api::errors::AppError;
use crate::api::routes::RouterState;
use axum::response::IntoResponse;
use axum::extract::{Form, Path, Query, State};
use axum::{http::StatusCode, response::Response, Json};
use serde::Deserialize;
use crate::api::models;

#[derive(Deserialize)]
pub struct ReplyRequest {
    pub channel_id: String,
    pub ts: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
    channel_id: Option<String>,
    user_id: Option<String>,
}

#[derive(Deserialize)]
pub struct Pagination {
    last_msg_timestamp: Option<String>,
    per_page: u32,
}

#[derive(Deserialize)]
pub struct DateQuery {
    since: Option<String>,
}

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
            5,
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

pub async fn get_messages(
    State(state): State<RouterState>,
    Path(channel_id): Path<String>,
    pagination: Query<Pagination>,
    date_query: Query<DateQuery>,
) -> Result<(StatusCode, Response), AppError> {
    let messages = state
        .tummy
        .fetch_msg_page(
            &channel_id,
            &pagination
                .last_msg_timestamp
                .as_ref()
                .map(|ts| sqlx::types::chrono::NaiveDateTime::from_pg_ts(ts)),
            &pagination.per_page,
            &date_query
                .since
                .as_ref()
                .map(|ts| sqlx::types::chrono::NaiveDateTime::from_pg_ts(ts))
                .unwrap_or(sqlx::types::chrono::DateTime::UNIX_EPOCH.naive_utc()),
        )
        .await?;

    let new_last_msg_timestamp = messages
        .last()
        .map(|message| message.timestamp)
        .unwrap_or(sqlx::types::chrono::DateTime::UNIX_EPOCH.naive_utc());
    Ok((
        StatusCode::OK,
        Json(
            models::MessagesResponse {
                messages,
                last_msg_timestamp: new_last_msg_timestamp.to_string(),
                channel_id,
            }
        ).into_response(),
    ))
}

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
