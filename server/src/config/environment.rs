use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    db_url: String,
    port: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            db_url: env::var("DATABASE_URL").expect("Config::new failed to load DATABASE_URL"),
            port: env::var("RUST_PORT").expect("Config::new failed to load RUST_PORT"),
        }
    }

    pub async fn init_db(&self) -> MySqlPool {
        MySqlPoolOptions::new()
            .max_connections(255)
            .connect(&self.db_url)
            .await
            .expect("Config::init_db failed to connect database")
    }

    pub fn api_url(&self) -> SocketAddr {
        let url: String = format!("{}:{}", "0.0.0.0", &self.port);
        url.parse()
            .expect("Config::api_url failed to parse api url")
    }
}
