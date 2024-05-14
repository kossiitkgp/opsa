use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Message {
    pub id: i32,
    pub channel_name: String,
    pub user_id: String,
    pub text: String,
    // Timestamp
    pub ts: String,
    // Thread timestamp
    pub thread_ts: String,
    // If it is a thread, id of the user who sent the parent message
    pub parent_user_id: String,
}

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub real_name: String,
    pub display_name: String,
    pub image_url: String,
    pub email: String,
    pub deleted: bool,
    pub is_bot: bool,
}

#[derive(FromRow)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub purpose: String,
}
