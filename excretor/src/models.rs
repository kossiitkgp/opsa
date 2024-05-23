use sqlx::types::chrono;
use crate::{dbmodels, tummy::SlackDateTime};

pub struct Message {
    pub channel_name: String,
    pub user_id: String,
    pub text: String,
    pub timestamp: chrono::NaiveDateTime,
    pub thread_timestamp: Option<chrono::NaiveDateTime>,
    pub parent_user_id: String,
    pub formatted_timestamp: String,
}

impl Message {
    pub fn from_db_message(db_message: &dbmodels::DBMessage) -> Self {
        Message {
            channel_name: db_message.channel_name.clone(),
            user_id: db_message.user_id.clone(),
            text: db_message.text.clone(),
            timestamp: db_message.timestamp,
            thread_timestamp: db_message.thread_timestamp,
            parent_user_id: db_message.parent_user_id.clone(),
            formatted_timestamp: db_message.timestamp.human_format(),
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
    pub fn from_db_user(db_user: &dbmodels::DBUser) -> Self {
        User {
            id: db_user.id.clone(),
            name: db_user.name.clone(),
            real_name: db_user.real_name.clone(),
            display_name: db_user.display_name.clone(),
            image_url: if db_user.image_url.is_empty() { "/assets/avatar.png".into() } else { db_user.image_url.clone() },
            email: db_user.email.clone(),
            deleted: db_user.deleted,
            is_bot: db_user.is_bot,
        }
    }
}

pub struct Channel {
    pub name: String,
    pub topic: String,
    pub purpose: String,
}

impl Channel {
    pub fn from_db_channel(db_channel: &dbmodels::DBChannel) -> Self {
        Channel {
            name: db_channel.name.clone(),
            topic: db_channel.topic.clone(),
            purpose: db_channel.purpose.clone(),
        }
    }
    
}