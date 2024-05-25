use sqlx::types::chrono;
use crate::{dbmodels, tummy::SlackDateTime};

pub struct Message {
    pub channel_id: String,
    pub user_id: String,
    pub text: String,
    pub timestamp: chrono::NaiveDateTime,
    pub thread_timestamp: Option<chrono::NaiveDateTime>,
    pub parent_user_id: Option<String>,
    pub formatted_timestamp: String,
}

impl Message {
    pub fn from_db_message_and_user(db_message_and_user: &dbmodels::DBMessageAndUser) -> Self {
        Message {
            channel_id: db_message_and_user.channel_id.clone(),
            user_id: db_message_and_user.user_id.clone(),
            text: db_message_and_user.msg_text.clone(),
            timestamp: db_message_and_user.ts,
            thread_timestamp: db_message_and_user.thread_ts,
            parent_user_id: db_message_and_user.parent_user_id.clone(),
            formatted_timestamp: db_message_and_user.ts.human_format(),
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

impl User {
    pub fn from_db_message_and_user(db_message_and_user: &dbmodels::DBMessageAndUser) -> Self {
        User {
            id: db_message_and_user.id.clone(),
            name: db_message_and_user.name.clone(),
            real_name: db_message_and_user.real_name.clone(),
            display_name: db_message_and_user.display_name.clone(),
            image_url: if let Some(url) = &db_message_and_user.image_url { url.clone() } else { "/assets/avatar.png".into() },
            email: db_message_and_user.email.clone(),
            deleted: db_message_and_user.deleted,
            is_bot: db_message_and_user.is_bot,
        }
    }
}

pub struct Channel {
    pub id: String,
    pub name: String,
    pub topic: String,
    pub purpose: String,
}

impl Channel {
    pub fn from_db_channel(db_channel: &dbmodels::DBChannel) -> Self {
        Channel {
            id: db_channel.id.clone(),
            name: db_channel.name.clone(),
            topic: db_channel.topic.to_owned().unwrap_or("".into()),
            purpose: db_channel.purpose.to_owned().unwrap_or("".into()),
        }
    }
    
}