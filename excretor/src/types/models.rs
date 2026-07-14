use crate::db::dbmodels::{DBChannel, DBParentMessage, DBReply, DBSearchResult, DBUser};
use crate::db::tummy::SlackDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

struct UserData<'a> {
    id: &'a str,
    name: &'a str,
    real_name: &'a str,
    display_name: &'a str,
    image_url: Option<&'a String>,
    email: &'a str,
    deleted: bool,
    is_bot: bool,
}

fn build_user(data: UserData<'_>) -> User {
    User {
        id: data.id.to_string(),
        name: data.name.to_string(),
        real_name: data.real_name.to_string(),
        display_name: data.display_name.to_string(),
        image_url: data
            .image_url
            .filter(|url| !url.is_empty())
            .map(|url| url.to_string())
            .unwrap_or_else(|| "/assets/avatar.png".into()),
        email: data.email.to_string(),
        deleted: data.deleted,
        is_bot: data.is_bot,
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
            user: build_user(UserData {
                id: &item.user_id,
                name: &item.name,
                real_name: &item.real_name,
                display_name: &item.display_name,
                image_url: item.image_url.as_ref(),
                email: &item.email,
                deleted: item.deleted,
                is_bot: item.is_bot,
            }),
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
            user: build_user(UserData {
                id: &item.user_id,
                name: &item.name,
                real_name: &item.real_name,
                display_name: &item.display_name,
                image_url: item.image_url.as_ref(),
                email: &item.email,
                deleted: item.deleted,
                is_bot: item.is_bot,
            }),
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
            user: build_user(UserData {
                id: &item.user_id,
                name: &item.name,
                real_name: &item.real_name,
                display_name: &item.display_name,
                image_url: item.image_url.as_ref(),
                email: &item.email,
                deleted: item.deleted,
                is_bot: item.is_bot,
            }),
        };

        let parent_message = if let (Some(parent_user_id), Some(parent_msg_text)) =
            (&item.parent_user_id, &item.parent_msg_text)
        {
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
                user: build_user(UserData {
                    id: parent_user_id,
                    name: item.parent_name.as_ref().unwrap(),
                    real_name: item.parent_real_name.as_ref().unwrap(),
                    display_name: item.parent_display_name.as_ref().unwrap(),
                    image_url: item.parent_image_url.as_ref(),
                    email: item.parent_email.as_ref().unwrap(),
                    deleted: item.parent_deleted.unwrap(),
                    is_bot: item.parent_is_bot.unwrap(),
                }),
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
        build_user(UserData {
            id: &item.id,
            name: &item.name,
            real_name: &item.real_name,
            display_name: &item.display_name,
            image_url: item.image_url.as_ref(),
            email: &item.email,
            deleted: item.deleted,
            is_bot: item.is_bot,
        })
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
