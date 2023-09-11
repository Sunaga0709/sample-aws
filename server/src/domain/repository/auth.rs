use async_trait::async_trait;

use crate::apperror::error::AppError;
use crate::domain::model::auth::EmailAndPass;
use crate::domain::model::auth::Token;

#[async_trait]
pub trait AuthRepository {
    async fn sign_up(&self, email: &str) -> Result<String, AppError>;
    async fn set_password(&self, param: &EmailAndPass) -> Result<(), AppError>;
    async fn sign_in(&self, param: &EmailAndPass) -> Result<Token, AppError>;
    async fn delete_user(&self, email: &str) -> Result<(), AppError>;
}
