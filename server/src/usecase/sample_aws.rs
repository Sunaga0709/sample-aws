use sqlx::mysql::MySqlPool;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;
use crate::domain::model::image::Image as ImageModel;
use crate::domain::repository::cloud_stroage::CloudStorageRepository;
use crate::domain::repository::example::ExampleRepository;

#[derive(Debug)]
pub struct SampleAwsUsecase<ER, CS>
where
    ER: ExampleRepository + Send + Sync,
    CS: CloudStorageRepository + Send + Sync,
{
    exm_repo: ER,
    cs_repo: CS,
    pool: MySqlPool,
}

impl<ER, CS> SampleAwsUsecase<ER, CS>
where
    ER: ExampleRepository + Send + Sync,
    CS: CloudStorageRepository + Send + Sync,
{
    pub fn new(exm_repo: ER, cs_repo: CS, pool: MySqlPool) -> Self {
        Self {
            exm_repo,
            cs_repo,
            pool,
        }
    }

    pub async fn test(&self) -> Result<(), AppError> {
        let exm: ExampleModel = ExampleModel::new();
        self.exm_repo.create(&self.pool, &exm).await?;
        Ok(())
    }

    pub async fn upload_image(&self, name: &str, image_data: &str) -> Result<(), AppError> {
        let new_image: ImageModel = ImageModel::new(name, image_data)?;

        self.cs_repo.upload(&new_image).await?;

        Ok(())
    }
}
