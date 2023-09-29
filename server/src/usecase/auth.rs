use sqlx::{Any, Pool};

use crate::apperror::error::AppError;
use crate::domain::model::auth::{EmailAndPass, Token};
use crate::domain::model::user::User as UserModel;
use crate::domain::repository::auth::AuthRepository;
use crate::domain::repository::user::UserRepository;

#[derive(Clone, Debug)]
pub struct AuthUsecase<AR, UR>
where
    AR: AuthRepository + Send + Sync,
    UR: UserRepository + Send + Sync,
{
    auth_repo: AR,
    user_repo: UR,
    pool: Pool<Any>,
}

impl<AR, UR> AuthUsecase<AR, UR>
where
    AR: AuthRepository + Send + Sync,
    UR: UserRepository + Send + Sync,
{
    pub fn new(auth_repo: AR, user_repo: UR, pool: Pool<Any>) -> Self {
        Self {
            auth_repo,
            user_repo,
            pool,
        }
    }

    pub async fn sign_up(
        &self,
        name: &str,
        email: &str,
        birthday: i64,
        password: &str,
        password_confirm: &str,
    ) -> Result<Token, AppError> {
        let mail_pass: EmailAndPass = EmailAndPass::new(email, password, password_confirm)?;
        let mut new_user: UserModel = UserModel::new(name, birthday, email)?;

        let user_opt: Option<UserModel> = self.user_repo.get_by_email(&self.pool, email).await?;
        if user_opt.is_some() {
            return Err(AppError::new_already_exists(
                "AuthUseCase::sign_up this email is already exists",
            ));
        }

        let auth_provider_id: String = self.auth_repo.sign_up(&mail_pass.email).await?;
        new_user.set_auth_provider_id(auth_provider_id);

        if let Err(err) = self.auth_repo.set_password(&mail_pass).await {
            self.auth_repo.delete_user(&mail_pass.email).await?;
            return Err(err);
        }

        let token: Token = match self.auth_repo.sign_in(&mail_pass).await {
            Ok(token) => token,
            Err(err) => {
                self.auth_repo.delete_user(&mail_pass.email).await?;
                return Err(err);
            }
        };

        if let Err(err) = self.user_repo.create(&self.pool, &new_user).await {
            self.auth_repo.delete_user(&mail_pass.email).await?;
            return Err(err);
        }

        Ok(token)
    }
}
