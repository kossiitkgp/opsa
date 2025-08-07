use serde::Serialize;
use crate::{types};

#[derive(Serialize)]
pub struct Channels {
    pub channels: Vec<types::Channel>,
}

