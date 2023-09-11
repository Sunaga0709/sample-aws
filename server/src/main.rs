use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;

mod apperror;
mod auth_service;
mod config;
mod domain;
mod gateway;
mod gen;
mod service;
mod usecase;
mod util;

use crate::auth_service::AuthService;
use crate::config::environment::Config;
use crate::gateway::aws_cognito::AwsCognito;
use crate::gateway::aws_s3::AwsS3;
use crate::gateway::example::ExampleRepoImpl;
use crate::gateway::user::UserRepoImpl;
use crate::gen::auth_sample_aws_service::auth_sample_aws_service_server::AuthSampleAwsServiceServer;
use crate::gen::auth_sample_aws_service::FILE_DESCRIPTOR_SET as AUTH_FILE_DESCRIPTOR_SET;
use crate::gen::sample_aws_service::sample_aws_service_server::SampleAwsServiceServer;
use crate::gen::sample_aws_service::FILE_DESCRIPTOR_SET;
use crate::service::SampleAws;
use crate::usecase::auth::AuthUsecase;
use crate::usecase::example::ExampleUsecase;
use crate::usecase::image::ImageUsecase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let conf: Config = Config::init();
    let pool: sqlx::Pool<sqlx::MySql> = conf.init_db().await;

    let auth_service: AuthSampleAwsServiceServer<AuthService<AwsCognito, UserRepoImpl>> =
        AuthSampleAwsServiceServer::new(AuthService::new(AuthUsecase::new(
            AwsCognito::new(
                &conf.aws_config().await,
                conf.user_pool_id(),
                conf.app_client_id(),
                conf.app_client_secret(),
            ),
            UserRepoImpl::new(),
            pool.clone(),
        )));

    let service: SampleAwsServiceServer<SampleAws<ExampleRepoImpl, AwsS3>> =
        SampleAwsServiceServer::new(SampleAws::new(
            ImageUsecase::new(AwsS3::new(&conf.aws_config().await, conf.s3_bucket_name())),
            ExampleUsecase::new(ExampleRepoImpl::new(), pool.clone()),
        ));

    let reflect = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(AUTH_FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .expect("");

    log::info!("start server");
    Server::builder()
        .add_service(reflect)
        .add_service(auth_service)
        .add_service(service)
        .serve(conf.api_url())
        .await?;

    Ok(())
}
