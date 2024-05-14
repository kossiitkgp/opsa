use clap::Parser;

#[derive(Parser)]
pub struct EnvVars {
    #[arg(env, default_value = "3000")]
    pub excretor_port: String,
    #[arg(env)]
    pub tummy_username: String,
    #[arg(env)]
    pub tummy_db: String,
    #[arg(env)]
    pub tummy_port: String,
    #[arg(env)]
    pub tummy_host: String,
    #[arg(env)]
    pub tummy_password: String,
}
