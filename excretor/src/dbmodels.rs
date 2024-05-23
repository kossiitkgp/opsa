use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};

// #[derive(Serialize, Deserialize)]
// pub struct DBMessage {
//     pub channel_name: String,
//     pub user_id: String,
//     pub msg_text: String,
//     pub ts: chrono::NaiveDateTime,
//     pub thread_ts: Option<chrono::NaiveDateTime>,
//     // If it is a thread, id of the user who sent the parent message
//     pub parent_user_id: Option<String>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct DBUser {
//     pub id: String,
//     pub name: String,
//     pub real_name: String,
//     pub display_name: String,
//     pub image_url: Option<String>,
//     pub email: String,
//     pub deleted: bool,
//     pub is_bot: bool,
// }

#[derive(Serialize, Deserialize)]
pub struct DBMessageAndUser {
    pub channel_name: String,
    pub user_id: String,
    pub msg_text: String,
    pub ts: chrono::NaiveDateTime,
    pub thread_ts: Option<chrono::NaiveDateTime>,
    pub parent_user_id: Option<String>,
    pub id: String,
    pub name: String,
    pub real_name: String,
    pub display_name: String,
    pub image_url: Option<String>,
    pub email: String,
    pub deleted: bool,
    pub is_bot: bool,
}

// #[derive(FromRow)]
// pub struct DBMessageAndUser {
//     #[sqlx(flatten)]
//     pub message: DBMessage,
//     #[sqlx(flatten)]
//     pub user: DBUser,
// }

#[derive(FromRow)]
pub struct DBChannel {
    pub name: String,
    pub topic: Option<String>,
    pub purpose: Option<String>,
}
