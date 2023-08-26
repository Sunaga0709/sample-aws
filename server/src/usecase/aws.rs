use sqlx::mysql::MySqlPool;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;
use crate::domain::repository::example::ExampleRepository;

#[derive(Debug)]
pub struct AwsUsecase<
    ER: ExampleRepository + Send + Sync
> {
    exm_repo: ER,
    pool: MySqlPool,
}

impl<ER> AwsUsecase<ER>
where
    ER: ExampleRepository + Send + Sync,
{
    pub fn new(exm_repo: ER, pool: MySqlPool) -> Self {
        Self { exm_repo, pool }
    }

    pub async fn test(&self) -> Result<(), AppError> {
        let exm: ExampleModel = ExampleModel::new();
        self.exm_repo.create(&self.pool, &exm).await?;
        Ok(())
    }
}
