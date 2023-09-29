use async_trait::async_trait;
use sqlx::Acquire;
use sqlx::Any;

use crate::apperror::error::AppError;
use crate::domain::model::user::User as UserModel;
use crate::domain::model::vector;
use crate::domain::repository::user::UserRepository;

#[derive(Clone, Debug)]
pub struct UserRepoImpl;

impl UserRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserRepository for UserRepoImpl {
    async fn get<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        user_id: &str,
    ) -> Result<Option<UserModel>, AppError> {
        let mut conn: <A as Acquire>::Connection = acq.acquire().await.map_err(|err| {
            AppError::new_internal_with_error(
                "UserRepoImpl::get failed to get database connection",
                &err,
            )
        })?;

        let rows: Vec<UserModel> = sqlx::query_as::<_, UserModel>(
            r#"
                SELECT
                    user_id,
                    auth_provider_id,
                    name,
                    birthday,
                    email,
                    created_at,
                    updated_at
                FROM
                    user
                WHERE
                    user_id = ?
                    AND is_deleted = FALSE
                LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_all(&mut *conn)
        .await
        .map_err(|err| {
            AppError::new_internal_with_error("UserRepoImpl::get failed to get user", &err)
        })?;

        Ok(vector::first(&rows))
    }

    async fn get_by_email<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        email: &str,
    ) -> Result<Option<UserModel>, AppError> {
        let mut conn: <A as Acquire>::Connection = acq.acquire().await.map_err(|err| {
            AppError::new_internal_with_error(
                "UserRepoImpl::get_by_email failed to get database connection",
                &err,
            )
        })?;

        let rows: Vec<UserModel> = sqlx::query_as::<_, UserModel>(
            r#"
                SELECT
                    user_id,
                    auth_provider_id,
                    name,
                    birthday,
                    email,
                    created_at,
                    updated_at
                FROM
                    user
                WHERE
                    email = ?
                    AND is_deleted = FALSE
                LIMIT 1
            "#,
        )
        .bind(email)
        .fetch_all(&mut *conn)
        .await
        .map_err(|err| {
            AppError::new_internal_with_error("UserRepoImpl::get_by_email failed to get user", &err)
        })?;

        Ok(vector::first(&rows))
    }

    async fn create<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        user: &UserModel,
    ) -> Result<(), AppError> {
        let mut conn: <A as Acquire>::Connection = acq.acquire().await.map_err(|err| {
            AppError::new_internal_with_error(
                "UserRepoImpl::create failed to get database connection",
                &err,
            )
        })?;

        sqlx::query(
            r#"
                INSERT INTO user (
                    user_id,
                    auth_provider_id,
                    name,
                    birthday,
                    email,
                    created_at,
                    updated_at
                ) VALUES (
                    ?, ?, ?, ?, ?, ?, ?
                )
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.auth_provider_id)
        .bind(&user.name)
        .bind(user.birthday)
        .bind(&user.email)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            AppError::new_internal_with_error("UserRepoImpl::create failed to create user", &err)
        })?;

        Ok(())
    }
}
