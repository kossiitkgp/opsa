use crate::models::{Channel, Message};
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
    pub messages: Vec<Message>,
    pub page: usize,
    pub channel: Channel,
}
