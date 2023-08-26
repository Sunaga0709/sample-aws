use async_trait::async_trait;
use sqlx::mysql::MySql;
use sqlx::Acquire;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;

#[async_trait]
pub trait ExampleRepository {
    async fn create<'a, A: Acquire<'a, Database = MySql> + Send + Sync>(
        &self,
        acq: A,
        exm: &ExampleModel,
    ) -> Result<(), AppError>;
}
