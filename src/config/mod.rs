pub struct Config {
    pub mongodb_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();

        Config {
            mongodb_url: env::var("MONGODB_URL").expect("MONGODB_URL must be set"),
        }
    }
}
