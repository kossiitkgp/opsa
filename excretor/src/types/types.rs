use serde::{Deserialize, Serialize};
use crate::{
    db::dbmodels::{DBChannel, DBParentMessage, DBReply, DBUser, DBSearchResult},
};
use sqlx::types::chrono;
use crate::db::tummy::SlackDateTime;


// This private helper centralizes the logic for creating a User struct.
// It now accepts references to avoid unnecessary cloning.
fn build_user(
    id: &str,
    name: &str,
    real_name: &str,
    display_name: &str,
    image_url: Option<&String>,
    email: &str,
    deleted: bool,
    is_bot: bool,
) -> User {
    User {
        id: id.to_string(),
        name: name.to_string(),
        real_name: real_name.to_string(),
        display_name: display_name.to_string(),
        // This logic now handles an Option<&String>.
        image_url: image_url
            .filter(|url| !url.is_empty())
            .map(|url| url.to_string())
            .unwrap_or_else(|| "/assets/avatar.png".into()),
        email: email.to_string(),
        deleted,
        is_bot,
    }
}


/// Represents a message in a channel, including user and thread information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// The ID of the channel where the message was posted.
    pub channel_id: String,
    pub channel_name: String,
    /// The ID of the user who posted the message.
    pub user_id: String,
    /// The message text content.
    pub text: String,
    /// The timestamp when the message was created.
    pub timestamp: chrono::NaiveDateTime,
    /// The timestamp of the parent thread, if this message is a reply.
    pub thread_timestamp: Option<chrono::NaiveDateTime>,
    /// The ID of the parent user, if applicable.
    pub parent_user_id: Option<String>,
    /// A human-readable formatted timestamp.
    pub formatted_timestamp: String,
    /// The number of replies in the thread.
    pub thread_count: i64,
    /// The user who posted the message.
    pub user: User,
}

/// Represents a search result, which includes the message and optionally its parent.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    #[serde(flatten)]
    pub message: Message,
    pub parent_message: Option<Box<Message>>,
}


/// Converts a `DBParentMessage` database model into a `Message`.
impl From<DBParentMessage> for Message {
    fn from(item: DBParentMessage) -> Self {
        Message {
            channel_id: item.channel_id,
            channel_name: item.channel_name,
            user_id: item.user_id.clone(), // Clone user_id for the message field
            text: item.msg_text,
            timestamp: item.ts,
            thread_timestamp: item.thread_ts,
            parent_user_id: item.parent_user_id,
            formatted_timestamp: item.ts.human_format(),
            thread_count: if let Some(thread_ts) = item.thread_ts {
                if item.ts == thread_ts {
                    item.cnt.unwrap_or(0) // It's a parent message
                } else {
                    0 // It's a reply
                }
            } else {
                0 // Not in a thread, so no thread count
            },
            user: build_user(
                &item.user_id,
                &item.name,
                &item.real_name,
                &item.display_name,
                item.image_url.as_ref(),
                &item.email,
                item.deleted,
                item.is_bot,
            ),
        }
    }
}

/// Converts a `DBReply` database model into a `Message`.
impl From<DBReply> for Message {
    fn from(item: DBReply) -> Self {
        Message {
            channel_id: item.channel_id,
            channel_name: item.channel_name,
            user_id: item.user_id.clone(), // Clone user_id for the message field
            text: item.msg_text,
            timestamp: item.ts,
            thread_timestamp: item.thread_ts,
            parent_user_id: item.parent_user_id,
            formatted_timestamp: item.ts.human_format(),
            thread_count: 0, // Replies always have a thread_count of 0
            user: build_user(
                &item.user_id,
                &item.name,
                &item.real_name,
                &item.display_name,
                item.image_url.as_ref(),
                &item.email,
                item.deleted,
                item.is_bot,
            ),
        }
    }
}

/// Converts a `DBSearchResult` into a `SearchResult`.
impl From<DBSearchResult> for SearchResult {
    fn from(item: DBSearchResult) -> Self {
        let message = Message {
            channel_id: item.channel_id,
            channel_name: item.channel_name,
            user_id: item.user_id.clone(),
            text: item.msg_text,
            timestamp: item.ts,
            thread_timestamp: item.thread_ts,
            parent_user_id: item.parent_user_id.clone(),
            formatted_timestamp: item.ts.human_format(),
            thread_count: if let Some(thread_ts) = item.thread_ts {
                if item.ts == thread_ts {
                    item.cnt.unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            },
            user: build_user(
                &item.user_id,
                &item.name,
                &item.real_name,
                &item.display_name,
                item.image_url.as_ref(),
                &item.email,
                item.deleted,
                item.is_bot,
            ),
        };

        let parent_message = if let (Some(parent_user_id), Some(parent_msg_text)) = (&item.parent_user_id, &item.parent_msg_text) {
            Some(Box::new(Message {
                channel_id: message.channel_id.clone(),
                channel_name: message.channel_name.clone(),
                user_id: parent_user_id.to_string(),
                text: parent_msg_text.to_string(),
                timestamp: item.thread_ts.unwrap(), // A reply must have a thread_ts
                thread_timestamp: item.thread_ts,
                parent_user_id: None, // The parent doesn't have a parent
                formatted_timestamp: item.thread_ts.unwrap().human_format(),
                thread_count: item.cnt.unwrap_or(0),
                user: build_user(
                    parent_user_id,
                    item.parent_name.as_ref().unwrap(),
                    item.parent_real_name.as_ref().unwrap(),
                    item.parent_display_name.as_ref().unwrap(),
                    item.parent_image_url.as_ref(),
                    item.parent_email.as_ref().unwrap(),
                    item.parent_deleted.unwrap(),
                    item.parent_is_bot.unwrap(),
                ),
            }))
        } else {
            None
        };

        SearchResult {
            message,
            parent_message,
        }
    }
}


/// Represents a user in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// The unique user ID.
    pub id: String,
    /// The username.
    pub name: String,
    /// The user's real name.
    pub real_name: String,
    /// The user's display name.
    pub display_name: String,
    /// The URL to the user's avatar image.
    pub image_url: String,
    /// The user's email address.
    pub email: String,
    /// Whether the user account is deleted.
    pub deleted: bool,
    /// Whether the user is a bot.
    pub is_bot: bool,
}

/// Converts a `DBUser` database model into a `User`.
/// This is now the single source of truth for converting a standalone DBUser.
impl From<DBUser> for User {
    fn from(item: DBUser) -> Self {
        // This also uses the helper to ensure consistency.
        build_user(
            &item.id,
            &item.name,
            &item.real_name,
            &item.display_name,
            item.image_url.as_ref(),
            &item.email,
            item.deleted,
            item.is_bot,
        )
    }
}

/// Represents a channel in the system.
#[derive(Serialize, Deserialize)]
pub struct Channel {
    /// The unique channel ID.
    pub id: String,
    /// The channel name.
    pub name: String,
    /// The channel topic.
    pub topic: String,
    /// The channel purpose.
    pub purpose: String,
}

/// Converts a `DBChannel` database model into a `Channel`.
impl From<DBChannel> for Channel {
    fn from(value: DBChannel) -> Self {
        Channel {
            id: value.id,
            name: value.name,
            topic: value.topic.unwrap_or_default(),
            purpose: value.purpose.unwrap_or_default(),
        }
    }
}
