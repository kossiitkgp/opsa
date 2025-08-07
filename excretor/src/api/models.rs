use crate::types::{Channel, Message};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChannelsResponse {
    pub channels: Vec<Channel>,
}

#[derive(Serialize)]
pub struct ChannelDetailsResponse {
    pub channel: Channel,
    pub messages: Vec<Message>,
    pub last_msg_timestamp: String,
    pub channel_id: String,
}

#[derive(Serialize)]
pub struct SearchResultsResponse {
    pub messages: Vec<Message>,
    pub query: String,
}

#[derive(Serialize)]
pub struct MessagesResponse {
    pub messages: Vec<Message>,
    pub last_msg_timestamp: String,
    pub channel_id: String,
}

#[derive(Serialize)]
pub struct ThreadResponse {
    pub messages: Vec<Message>,
    pub parent_ts: String,
    pub channel_id: String,
    pub parent_user_id: String,
}
