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
    pub fn from_db_message(db_message: dbmodels::DBMessage) -> Self {
        Message {
            channel_name: db_message.channel_name,
            user_id: db_message.user_id,
            text: db_message.text,
            timestamp: db_message.timestamp,
            thread_timestamp: db_message.thread_timestamp,
            parent_user_id: db_message.parent_user_id,
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
    pub fn from_db_user(db_user: dbmodels::DBUser) -> Self {
        User {
            id: db_user.id,
            name: db_user.name,
            real_name: db_user.real_name,
            display_name: db_user.display_name,
            image_url: if db_user.image_url.is_empty() { "/assets/avatar.png".into() } else { db_user.image_url },
            email: db_user.email,
            deleted: db_user.deleted,
            is_bot: db_user.is_bot,
        }
    }
}