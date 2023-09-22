use sqlx::FromRow;

use super::datetime;
use super::mail;
use super::uuid;
use crate::apperror::error::AppError;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    pub user_id: String,
    pub auth_provider_id: String,
    pub name: String,
    pub birthday: i64,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn new(name: &str, birthday: i64, email: &str) -> Result<Self, AppError> {
        Self::validate_name(name)?;
        Self::validate_birthday(birthday)?;
        Self::validate_email(email)?;

        let now: i64 = datetime::now_unix();

        Ok(Self {
            user_id: uuid::new_uuid(),
            auth_provider_id: String::new(),
            name: name.to_owned(),
            birthday,
            email: email.to_owned(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn set_auth_provider_id(&mut self, id: String) {
        self.auth_provider_id = id
    }

    fn validate_name(name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            Err(AppError::new_invalid_argument(
                "User::validate_name name is empty",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_birthday(day: i64) -> Result<(), AppError> {
        if datetime::now_unix() < day {
            Err(AppError::new_invalid_argument(
                "User::validate_birthday birthday is future date",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_email(mail: &str) -> Result<(), AppError> {
        let is_ok_mail: bool = mail::is_match_mail_format(mail)?;
        if is_ok_mail {
            Ok(())
        } else {
            Err(AppError::new_invalid_argument(
                "User::validate_email invalid email format",
            ))
        }
    }
}
