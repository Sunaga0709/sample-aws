use async_trait::async_trait;
use sqlx::Acquire;
use sqlx::Any;

use crate::apperror::error::AppError;
use crate::domain::model::user::User as UserModel;

#[async_trait]
pub trait UserRepository {
    async fn get<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        user_id: &str,
    ) -> Result<Option<UserModel>, AppError>;
    async fn get_by_email<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        email: &str,
    ) -> Result<Option<UserModel>, AppError>;
    async fn create<'a, A: Acquire<'a, Database = Any> + Send + Sync>(
        &self,
        acq: A,
        user: &UserModel,
    ) -> Result<(), AppError>;
}
