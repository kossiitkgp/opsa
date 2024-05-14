use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Message {
    pub channel_name: String,
    pub user_id: String,
    pub text: String,
    #[sqlx(rename = "ts")]
    pub timestamp: String,
    #[sqlx(rename = "thread_ts")]
    pub thread_timestamp: String,
    // If it is a thread, id of the user who sent the parent message
    pub parent_user_id: String,
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

#[derive(FromRow)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub purpose: String,
}
