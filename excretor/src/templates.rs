use crate::models::{Message, User, Channel};
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
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

fn generate_trigger (messages: &[(Message, User)]) -> bool {
    messages.len() == 10
}

#[derive(Template)]
#[template(path = "channel_page.html")]
pub struct ChannelPageTemplate {
    pub messages: Vec<(Message, User)>,
    pub last_msg_timestamp: String,
    pub channel_name: String,
}

#[derive(Template)]
#[template(path = "fallback_avatar.html")]
pub struct FallbackAvatarTemplate;
