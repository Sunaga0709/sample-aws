use aws_config::SdkConfig;
use sqlx::{Any, Pool};

use crate::gateway::example::ExampleRepoImpl;
use crate::usecase::example::ExampleUsecase;

use crate::gateway::aws_s3::AwsS3;
use crate::usecase::image::ImageUsecase;

pub fn di_example(pool: Pool<Any>) -> ExampleUsecase<ExampleRepoImpl> {
    ExampleUsecase::new(ExampleRepoImpl::new(), pool)
}

pub fn di_image(client: &SdkConfig, bucket: &str) -> ImageUsecase<AwsS3> {
    ImageUsecase::new(AwsS3::new(client, bucket))
}
