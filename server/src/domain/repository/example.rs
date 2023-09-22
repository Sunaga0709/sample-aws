use async_trait::async_trait;
use sqlx::Any;
use sqlx::Acquire;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;

#[async_trait]
pub trait ExampleRepository {
    async fn create<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        exm: &ExampleModel,
    ) -> Result<(), AppError>;
}
