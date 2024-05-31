use crate::{
    dbmodels::{DBChannel, DBParentMessage, DBReply},
    tummy::SlackDateTime,
};
use sqlx::types::chrono;

pub struct Message {
    pub channel_id: String,
    pub user_id: String,
    pub text: String,
    pub timestamp: chrono::NaiveDateTime,
    pub thread_timestamp: Option<chrono::NaiveDateTime>,
    pub parent_user_id: Option<String>,
    pub formatted_timestamp: String,
    pub thread_count: i64,
    pub user: User,
}

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

pub struct User {
    pub id: String,
    pub name: String,
    pub real_name: String,
    pub display_name: String,
    pub image_url: String,
    pub email: String,
    pub deleted: bool,
    pub is_bot: bool,
}

pub struct Channel {
    pub id: String,
    pub name: String,
    pub topic: String,
    pub purpose: String,
}

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
