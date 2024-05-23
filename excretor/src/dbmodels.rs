use sqlx::{prelude::FromRow, types::chrono};

#[derive(FromRow)]
pub struct DBMessage {
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
}

#[derive(FromRow)]
pub struct DBUser {
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
pub struct DBMessageAndUser {
    #[sqlx(flatten)]
    pub message: DBMessage,
    #[sqlx(flatten)]
    pub user: DBUser,
}

#[derive(FromRow)]
pub struct DBChannel {
    pub name: String,
    pub topic: String,
    pub purpose: String,
}
