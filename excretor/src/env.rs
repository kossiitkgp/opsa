struct EnvVars {
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
            excretor_port: std::env::var_os("EXCRETOR_PORT"),
        }
    }
}
