use async_trait::async_trait;
use sqlx::{Acquire, Any, Error as DBError};

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
    async fn get<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
    ) -> Result<Vec<ExampleModel>, AppError> {
        let mut conn: <A as Acquire>::Connection = acq.acquire().await.map_err(|err| {
            AppError::new_internal_with_error(
                "ExampleRepoImpl::get failed to get database connection",
                &err,
            )
        })?;

        sqlx::query_as::<_, ExampleModel>(
            r#"
                SELECT
                    example_id,
                    created_at
                FROM
                    example
            "#,
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|err| {
            AppError::new_internal_with_error("ExampleRepoImpl::get failed to get example", &err)
        })
    }

    async fn get_by_id<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        id: &str,
    ) -> Result<ExampleModel, AppError> {
        let mut conn: <A as Acquire>::Connection = acq.acquire().await.map_err(|err| {
            AppError::new_internal_with_error(
                "ExampleRepoImpl::get_by_id failed to get database connection",
                &err,
            )
        })?;

        let result: Result<ExampleModel, DBError> = sqlx::query_as::<_, ExampleModel>(
            r#"
                SELECT
                    example_id,
                    created_at
                FROM
                    example
                WHERE
                    example_id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&mut *conn)
        .await;

        match result {
            Ok(example) => Ok(example),
            Err(err) => match err {
                DBError::RowNotFound => Err(AppError::new_not_found_with_error(
                    "ExampleRepoImpl::get_by_id example not found",
                    &err,
                )),
                _ => Err(AppError::new_internal_with_error(
                    "ExampleRepoImpl::get_by_id failed to get example",
                    &err,
                )),
            },
        }
    }

    async fn create<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
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
