use async_trait::async_trait;

use crate::apperror::error::AppError;
use crate::domain::model::image::Image as ImageModel;

#[async_trait]
pub trait CloudStorageRepository {
    async fn upload(&self, image: &ImageModel) -> Result<(), AppError>;
}
