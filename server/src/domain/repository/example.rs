use async_trait::async_trait;
use sqlx::Acquire;
use sqlx::Any;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;

#[async_trait]
pub trait ExampleRepository {
    async fn get<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
    ) -> Result<Vec<ExampleModel>, AppError>;
    async fn get_by_id<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        id: &str,
    ) -> Result<ExampleModel, AppError>;
    async fn create<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        exm: &ExampleModel,
    ) -> Result<(), AppError>;
}
