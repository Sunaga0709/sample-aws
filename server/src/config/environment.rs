use aws_config::meta::region::RegionProviderChain;
use aws_types::SdkConfig;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::net::SocketAddr;

pub struct Config {
    db_url: String,
    port: String,
    aws_s3_bucket: String,
}

impl Config {
    pub fn init() -> Self {
        Self {
            db_url: env::var("DATABASE_URL").expect("Config::init failed to load DATABASE_URL"),
            port: env::var("RUST_PORT").expect("Config::init failed to load RUST_PORT"),
            aws_s3_bucket: env::var("AWS_S3_BUCKET")
                .expect("Config::init failed to load AWS_S3_BUCKET"),
        }
    }

    pub async fn init_db(&self) -> MySqlPool {
        MySqlPoolOptions::new()
            .max_connections(255)
            .connect(&self.db_url)
            .await
            .expect("Config::init_db failed to connect database")
    }

    pub async fn aws_config(&self) -> SdkConfig {
        let region_provider: RegionProviderChain =
            RegionProviderChain::default_provider().or_else("ap-northeast-1");
        aws_config::from_env().region(region_provider).load().await
    }

    pub fn s3_bucket_name(&self) -> &str {
        &self.aws_s3_bucket
    }

    pub fn api_url(&self) -> SocketAddr {
        let url: String = format!("{}:{}", "0.0.0.0", &self.port);
        url.parse()
            .expect("Config::api_url failed to parse api url")
    }
}
