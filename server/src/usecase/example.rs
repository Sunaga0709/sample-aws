use sqlx::{Any, Pool};

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;
use crate::domain::repository::example::ExampleRepository;

#[derive(Clone, Debug)]
pub struct ExampleUsecase<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    example_repo: ER,
    pool: Pool<Any>,
}

impl<ER> ExampleUsecase<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    pub fn new(example_repo: ER, pool: Pool<Any>) -> Self {
        Self { example_repo, pool }
    }

    pub async fn get(&self) -> Result<Vec<ExampleModel>, AppError> {
        self.example_repo.get(&self.pool).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ExampleModel, AppError> {
        self.example_repo.get_by_id(&self.pool, id).await
    }

    pub async fn create(&self) -> Result<(), AppError> {
        let new_example: ExampleModel = ExampleModel::new();
        self.example_repo.create(&self.pool, &new_example).await?;
        Ok(())
    }
}
