use askama::Template;
use crate::models::{Message, Channel};

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate{
    pub channels: Vec<Channel>,
    pub per_page: usize,
}

#[derive(Template)]
#[template(path = "channel.html")]
pub struct ChannelTemplate {
    pub channel: Channel,
    pub per_page: usize,
    pub messages: Vec<Message>,
}

#[derive(Template)]
#[template(path = "channel_page.html")]
pub struct ChannelPageTemplate {
    pub messages: Vec<Message>,
    pub page: usize,
    pub channel: Channel,
    pub per_page: usize,
}
