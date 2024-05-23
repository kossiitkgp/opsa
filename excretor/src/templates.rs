use crate::models::{Channel, MessageAndUser};
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

#[derive(Template)]
#[template(path = "channel_page.html")]
pub struct ChannelPageTemplate {
    pub messages: Vec<MessageAndUser>,
    pub last_msg_timestamp: String,
    pub channel_id: String,
}

#[derive(Template)]
#[template(path = "fallback_avatar.html")]
pub struct FallbackAvatarTemplate;
