use crate::db::tummy::SlackDateTime;
use crate::api::errors::AppError;
use crate::api::routes::RouterState;
use axum::response::IntoResponse;
use crate::templates;
use askama::Template;
use axum::extract::{Form, Path, Query, State};
use axum::{http::StatusCode, response::{Html, Response}, Json};
use serde::Deserialize;
use crate::api::handlers::handlers::ReplyRequest;

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

pub(super) async fn search(
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
    todo!();
    Ok((
        StatusCode::OK,
        Json(
            templates::SearchResultsTemplate {
                messages,
                query: payload.query,
            }
        ).into_response()
    ))
}

pub(super) async fn get_messages(
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
    todo!();
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
    todo!();
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
