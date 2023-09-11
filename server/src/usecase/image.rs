use crate::apperror::error::AppError;
use crate::domain::model::image::Image as ImageModel;
use crate::domain::repository::cloud_stroage::CloudStorageRepository;

#[derive(Debug)]
pub struct ImageUsecase<CS>
where
    CS: CloudStorageRepository + Send + Sync,
{
    cs_repo: CS,
}

impl<CS> ImageUsecase<CS>
where
    CS: CloudStorageRepository + Send + Sync,
{
    pub fn new(cs_repo: CS) -> Self {
        Self { cs_repo }
    }

    pub async fn upload_image(&self, name: &str, image_data: &str) -> Result<(), AppError> {
        let new_image: ImageModel = ImageModel::new(name, image_data)?;
        self.cs_repo.upload(&new_image).await?;
        Ok(())
    }
}
