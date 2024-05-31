use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
pub struct EnvVars {
    #[arg(env)]
    pub tummy_username: String,
    #[arg(env)]
    pub tummy_password: String,
    #[arg(env)]
    pub tummy_port: String,
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
