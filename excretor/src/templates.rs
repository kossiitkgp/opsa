use crate::models::{Channel, Message};
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub description: String,
    pub channels: Vec<Channel>,
}

#[derive(Template)]
#[template(path = "err.html")]
pub struct ErrTemplate {
    pub err_string: String,
}

#[derive(Template)]
#[template(path = "channel.html")]
pub struct ChannelTemplate {
    pub channel: Channel,
}

fn generate_trigger(messages: &[Message]) -> bool {
    messages.len() == 10
}

#[derive(Template)]
#[template(path = "channel_page.html")]
pub struct ChannelPageTemplate {
    pub messages: Vec<Message>,
    pub last_msg_timestamp: String,
    pub channel_id: String,
}

#[derive(Template)]
#[template(path = "thread.html")]
pub struct ThreadTemplate {
    pub messages: Vec<Message>,
    pub parent_ts: String,
    pub channel_id: String,
    pub parent_user_id: String,
}

#[derive(Template)]
#[template(path = "fallback_avatar.html")]
pub struct FallbackAvatarTemplate;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;
