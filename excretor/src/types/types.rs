use serde::{Deserialize, Serialize};
use crate::{
    db::dbmodels::{DBChannel, DBParentMessage, DBReply, DBUser},
};
use sqlx::types::chrono;
use crate::db::tummy::SlackDateTime;

/// Represents a message in a channel, including user and thread information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// The ID of the channel where the message was posted.
    pub channel_id: String,
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

/// Converts a `DBParentMessage` database model into a `Message`.
impl From<DBParentMessage> for Message {
    fn from(item: DBParentMessage) -> Self {
        Message {
            channel_id: item.channel_id,
            user_id: item.user_id,
            text: item.msg_text,
            timestamp: item.ts,
            thread_timestamp: item.thread_ts,
            parent_user_id: item.parent_user_id,
            formatted_timestamp: item.ts.human_format(),
            thread_count: item.cnt.unwrap_or(0),
            user: User {
                id: item.id,
                name: item.name,
                real_name: item.real_name,
                display_name: item.display_name,
                image_url: if let Some(url) = item.image_url {
                    if !url.is_empty() {
                        url
                    } else {
                        "/assets/avatar.png".into()
                    }
                } else {
                    "/assets/avatar.png".into()
                },
                email: item.email,
                deleted: item.deleted,
                is_bot: item.is_bot,
            },
        }
    }
}

/// Converts a `DBReply` database model into a `Message`.
impl From<DBReply> for Message {
    fn from(item: DBReply) -> Self {
        Message {
            channel_id: item.channel_id,
            user_id: item.user_id,
            text: item.msg_text,
            timestamp: item.ts,
            thread_timestamp: item.thread_ts,
            parent_user_id: item.parent_user_id,
            formatted_timestamp: item.ts.human_format(),
            thread_count: 0,
            user: User {
                id: item.id,
                name: item.name,
                real_name: item.real_name,
                display_name: item.display_name,
                image_url: if let Some(url) = item.image_url {
                    if !url.is_empty() {
                        url
                    } else {
                        "/assets/avatar.png".into()
                    }
                } else {
                    "/assets/avatar.png".into()
                },
                email: item.email,
                deleted: item.deleted,
                is_bot: item.is_bot,
            },
        }
    }
}

/// Represents a user in the system.
#[derive(Serialize, Deserialize, Debug)]
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
impl From<DBUser> for User {
    fn from(item: DBUser) -> Self {
        User {
            id: item.id,
            name: item.name,
            real_name: item.real_name,
            display_name: item.display_name,
            image_url: if let Some(url) = item.image_url {
                if !url.is_empty() {
                    url
                } else {
                    "/assets/avatar.png".into()
                }
            } else {
                "/assets/avatar.png".into()
            },
            email: item.email,
            deleted: item.deleted,
            is_bot: item.is_bot,
        }
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
            topic: value.topic.unwrap_or("".into()),
            purpose: value.purpose.unwrap_or("".into()),
        }
    }
}