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
use crate::gateway::aws_s3::AwsS3;
use crate::gateway::example::ExampleRepoImpl;
use crate::gen::sample_aws_v1::sample_aws_service_server::SampleAwsServiceServer;
use crate::service::sample_aws::Aws;
use crate::usecase::sample_aws::SampleAwsUsecase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let conf: Config = Config::init();
    let pool: sqlx::Pool<sqlx::MySql> = conf.init_db().await;
    let aws_service: SampleAwsServiceServer<Aws<ExampleRepoImpl, AwsS3>> =
        SampleAwsServiceServer::new(Aws::new(SampleAwsUsecase::new(
            ExampleRepoImpl::new(),
            AwsS3::new(&conf.aws_config().await, conf.s3_bucket_name()),
            pool,
        )));

    log::info!("start server");
    Server::builder()
        .add_service(aws_service)
        .serve(conf.api_url())
        .await?;
    Ok(())
}
