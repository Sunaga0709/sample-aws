use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_s3::Client as S3Client;

use crate::apperror::error::AppError;
use crate::domain::model::image::Image as ImageModel;
use crate::domain::repository::cloud_stroage::CloudStorageRepository;

#[derive(Debug)]
pub struct AwsS3 {
    client: S3Client,
    bucket: String,
}

impl AwsS3 {
    pub fn new(conf: &SdkConfig, bucket: &str) -> Self {
        Self {
            client: S3Client::new(conf),
            bucket: bucket.to_owned(),
        }
    }
}

#[async_trait]
impl CloudStorageRepository for AwsS3 {
    async fn upload(&self, image: &ImageModel) -> Result<(), AppError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&image.name)
            .body(image.decode_data.clone().into())
            .content_type("image/png")
            .send()
            .await
            .map_err(|err| {
                AppError::new_internal_with_error(
                    "AwsS3::upload failed to upload image to aws s3",
                    &err,
                )
            })?;
        Ok(())
    }
}
