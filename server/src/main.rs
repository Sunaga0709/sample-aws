use env_logger;
use tonic::transport::Server;

mod apperror;
mod config;
mod domain;
mod gateway;
mod gen;
mod service;
mod usecase;
mod util;

use crate::config::environment::Config;
use crate::gateway::example::ExampleRepoImpl;
use crate::gen::aws_v1::aws_service_server::AwsServiceServer;
use crate::service::aws::Aws;
use crate::usecase::aws::AwsUsecase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let conf: Config = Config::new();
    let pool: sqlx::Pool<sqlx::MySql> = conf.init_db().await;
    let aws_service: AwsServiceServer<Aws<ExampleRepoImpl>> =
        AwsServiceServer::new(Aws::new(AwsUsecase::new(ExampleRepoImpl::new(), pool)));

    log::info!("start server");
    Server::builder()
        .add_service(aws_service)
        .serve(conf.api_url())
        .await?;
    Ok(())
}
