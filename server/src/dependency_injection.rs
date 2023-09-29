use aws_config::SdkConfig;
use sqlx::{Any, Pool};

use crate::gateway::example::ExampleRepoImpl;
use crate::usecase::example::ExampleUsecase;

use crate::gateway::aws_s3::AwsS3;
use crate::usecase::image::ImageUsecase;

use crate::gateway::aws_cognito::AwsCognito;
use crate::gateway::user::UserRepoImpl;
use crate::usecase::auth::AuthUsecase;

pub fn di_example(pool: Pool<Any>) -> ExampleUsecase<ExampleRepoImpl> {
    ExampleUsecase::new(ExampleRepoImpl::new(), pool)
}

pub fn di_image(conf: &SdkConfig, bucket: &str) -> ImageUsecase<AwsS3> {
    ImageUsecase::new(AwsS3::new(conf, bucket))
}

pub fn di_auth(
    conf: &SdkConfig,
    user_pool_id: &str,
    app_client_id: &str,
    app_client_secret: &str,
    pool: Pool<Any>,
) -> AuthUsecase<AwsCognito, UserRepoImpl> {
    AuthUsecase::new(
        AwsCognito::new(conf, user_pool_id, app_client_id, app_client_secret),
        UserRepoImpl::new(),
        pool,
    )
}
