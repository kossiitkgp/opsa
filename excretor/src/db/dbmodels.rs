//! Database models for channels, users, parent messages, and replies.
//! These structs represent rows fetched from the database and are used for data conversion.

use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

/// Represents a channel record in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBChannel {
    /// The unique channel ID.
    pub id: String,
    /// The channel name.
    pub name: String,
    /// The channel topic, if set.
    pub topic: Option<String>,
    /// The channel purpose, if set.
    pub purpose: Option<String>,
}

/// Represents a user record in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBUser {
    /// The unique user ID.
    pub id: String,
    /// The username.
    pub name: String,
    /// The user's real name.
    pub real_name: String,
    /// The user's display name.
    pub display_name: String,
    /// The URL to the user's avatar image, if set.
    pub image_url: Option<String>,
    /// The user's email address.
    pub email: String,
    /// Whether the user account is deleted.
    pub deleted: bool,
    /// Whether the user is a bot.
    pub is_bot: bool,
}

/// Represents a parent message (thread root) in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBParentMessage {
    /// The channel ID where the message was posted.
    pub channel_id: String,
    pub channel_name: String,
    /// The user ID who posted the message.
    pub user_id: String,
    /// The message text content.
    pub msg_text: String,
    /// The timestamp when the message was created.
    pub ts: chrono::NaiveDateTime,
    /// The timestamp of the parent thread, if this message is a reply.
    pub thread_ts: Option<chrono::NaiveDateTime>,
    /// The ID of the parent user, if applicable.
    pub parent_user_id: Option<String>,
    /// The number of replies in the thread, if available.
    pub cnt: Option<i64>,
    /// The unique user ID (for joined user data).
    pub id: String,
    /// The username (for joined user data).
    pub name: String,
    /// The user's real name (for joined user data).
    pub real_name: String,
    /// The user's display name (for joined user data).
    pub display_name: String,
    /// The URL to the user's avatar image, if set.
    pub image_url: Option<String>,
    /// The user's email address.
    pub email: String,
    /// Whether the user account is deleted.
    pub deleted: bool,
    /// Whether the user is a bot.
    pub is_bot: bool,
}

/// Represents a reply message in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBReply {
    /// The channel ID where the reply was posted.
    pub channel_id: String,
    pub channel_name: String,
    /// The user ID who posted the reply.
    pub user_id: String,
    /// The reply text content.
    pub msg_text: String,
    /// The timestamp when the reply was created.
    pub ts: chrono::NaiveDateTime,
    /// The timestamp of the parent thread.
    pub thread_ts: Option<chrono::NaiveDateTime>,
    /// The ID of the parent user, if applicable.
    pub parent_user_id: Option<String>,
    /// The unique user ID (for joined user data).
    pub id: String,
    /// The username (for joined user data).
    pub name: String,
    /// The user's real name (for joined user data).
    pub real_name: String,
    /// The user's display name (for joined user data).
    pub display_name: String,
    /// The URL to the user's avatar image, if set.
    pub image_url: Option<String>,
    /// The user's email address.
    pub email: String,
    /// Whether the user account is deleted.
    pub deleted: bool,
    /// Whether the user is a bot.
    pub is_bot: bool,
}

/// Represents a search result, which could be a parent message or a reply.
/// If it's a reply, it includes information about the parent message.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSearchResult {
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub msg_text: String,
    pub ts: chrono::NaiveDateTime,
    pub thread_ts: Option<chrono::NaiveDateTime>,
    pub parent_user_id: Option<String>,
    pub cnt: Option<i64>,

    pub id: String,
    pub name: String,
    pub real_name: String,
    pub display_name: String,
    pub image_url: Option<String>,
    pub email: String,
    pub deleted: bool,
    pub is_bot: bool,

    // opt parent message fields (for replies)
    pub parent_msg_text: Option<String>,
    pub parent_name: Option<String>,
    pub parent_real_name: Option<String>,
    pub parent_display_name: Option<String>,
    pub parent_image_url: Option<String>,
    pub parent_email: Option<String>,
    pub parent_deleted: Option<bool>,
    pub parent_is_bot: Option<bool>,
}
