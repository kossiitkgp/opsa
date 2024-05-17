use sqlx::{prelude::FromRow, types::chrono};

use crate::tummy::SlackDateTime;

#[derive(FromRow)]
pub struct Message {
    pub channel_name: String,
    pub user_id: String,
    #[sqlx(rename = "msg_text")]
    pub text: String,
    #[sqlx(rename = "ts")]
    pub timestamp: chrono::NaiveDateTime,
    #[sqlx(rename = "thread_ts")]
    pub thread_timestamp: Option<chrono::NaiveDateTime>,
    // If it is a thread, id of the user who sent the parent message
    pub parent_user_id: String,
    #[sqlx(skip)]
    pub formatted_timestamp: String,
}

impl Message {
    pub fn set_formatted_timestamp(&mut self) {
        self.formatted_timestamp = self.timestamp.human_format();
    }
}

#[derive(FromRow)]
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

#[derive(FromRow)]
pub struct MessageAndUser {
    #[sqlx(flatten)]
    pub message: Message,
    #[sqlx(flatten)]
    pub user: User,
}

impl MessageAndUser {
    pub fn set_formatted_timestamp(&mut self) {
        self.message.set_formatted_timestamp();
    }
}

#[derive(FromRow)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub purpose: String,
}
