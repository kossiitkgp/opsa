pub struct EnvVars {
    pub excretor_port: String,
    pub tummy_username: String,
    pub tummy_db: String,
    pub tummy_port: String,
    pub tummy_host: String,
    pub tummy_password: String,
}

impl EnvVars {
    pub fn init() -> Self {
        Self {
            excretor_port: std::env::var_os("EXCRETOR_PORT")
                .unwrap_or("3000".into())
                .into_string()
                .unwrap(),
            tummy_username: std::env::var("TUMMY_USERNAME").unwrap(),
            tummy_db: std::env::var("TUMMY_DB").unwrap(),
            tummy_port: std::env::var("TUMMY_PORT").unwrap(),
            tummy_host: std::env::var("TUMMY_HOST").unwrap(),
            tummy_password: std::env::var("TUMMY_PASSWORD").unwrap(),
        }
    }
}
