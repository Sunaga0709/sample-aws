use sqlx::mysql::MySqlPool;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;
use crate::domain::repository::example::ExampleRepository;

#[derive(Debug)]
pub struct ExampleUsecase<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    example_repo: ER,
    pool: MySqlPool,
}

impl<ER> ExampleUsecase<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    pub fn new(example_repo: ER, pool: MySqlPool) -> Self {
        Self { example_repo, pool }
    }

    pub async fn example(&self) -> Result<(), AppError> {
        let new_example: ExampleModel = ExampleModel::new();
        self.example_repo.create(&self.pool, &new_example).await?;
        Ok(())
    }
}
