use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
#[clap(name = "tummy")]
pub struct EnvVars {
    #[arg(env)]
    pub tummy_username: String,
    #[arg(env)]
    pub tummy_password: String,
    #[arg(env)]
    pub tummy_port: String,
    #[arg(env)]
    pub slack_client_id: String,
    #[arg(env)]
    pub slack_client_secret: String,
    #[arg(env)]
    pub slack_signing_secret: String,
    #[arg(env)]
    pub slack_redirect_uri: String,
    #[arg(env, default_value = "false", action = clap::ArgAction::Set)]
    pub slack_auth_enable: bool,
    #[arg(env, default_value = "30")]
    pub keep_logged_in_for_days: i64,
    #[arg(env, default_value = "postgres://localhost/tummy")]
    pub database_url: String,
    #[arg(env, default_value = "assets/")]
    pub static_assets_dir: PathBuf,
    #[arg(env, default_value = "tummy")]
    pub tummy_db: String,
    #[arg(env, default_value = "localhost")]
    pub tummy_host: String,
    #[arg(env, default_value = "3000")]
    pub excretor_port: String,
    #[arg(env, default_value = "OPSA")]
    pub title: String,
    #[arg(env, default_value = "Our Precious Slack Archive")]
    pub description: String,
}

impl EnvVars {
    /// Processes the environment variables after reading.
    pub fn process(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        self.static_assets_dir = self.static_assets_dir.canonicalize()?;
        Ok(self)
    }
}
