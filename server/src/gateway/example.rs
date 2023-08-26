use async_trait::async_trait;
use sqlx::mysql::MySql;
use sqlx::Acquire;

use crate::apperror::error::AppError;
use crate::domain::model::example::Example as ExampleModel;
use crate::domain::repository::example::ExampleRepository;

#[derive(Clone, Debug)]
pub struct ExampleRepoImpl {}

impl ExampleRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ExampleRepository for ExampleRepoImpl {
    async fn create<'a, A: Acquire<'a, Database = MySql> + Send + Sync>(
        &self,
        acq: A,
        exm: &ExampleModel,
    ) -> Result<(), AppError> {
        let mut conn: <A as Acquire>::Connection = acq.acquire().await.map_err(|err| {
            AppError::new_internal_with_error(
                "ExampleRepoImpl::create failed to get database connection",
                &err,
            )
        })?;

        sqlx::query(
            r#"
                INSERT INTO example (
                    example_id, created_at
                ) VALUES (
                    ?, ?
                )
            "#,
        )
        .bind(&exm.example_id)
        .bind(exm.created_at)
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            AppError::new_internal_with_error("ExampleRepoImpl::create failed to create", &err)
        })?;

        Ok(())
    }
}
