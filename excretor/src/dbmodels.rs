use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DBParentMessage {
    pub channel_id: String,
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
    pub cnt: Option<i64>,
}

#[derive(Deserialize)]
pub struct DBReply {
    pub channel_id: String,
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

#[derive(Serialize, Deserialize)]
pub struct DBChannel {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    pub purpose: Option<String>,
}
