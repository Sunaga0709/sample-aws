use crate::apperror::error::AppError;
use crate::util::mail;

const MIN_PASSWORD_LENGTH: usize = 7;

#[derive(Clone, Debug)]
pub struct EmailAndPass {
    pub email: String,
    pub password: String,
}

impl EmailAndPass {
    pub fn new(email: &str, password: &str, password_confirm: &str) -> Result<Self, AppError> {
        Self::validate_email(email)?;
        Self::validate_password(password, password_confirm)?;

        Ok(Self {
            email: email.to_owned(),
            password: password.to_owned(),
        })
    }

    fn validate_email(mail: &str) -> Result<(), AppError> {
        let is_ok_mail: bool = mail::is_match_mail_format(mail)?;
        if is_ok_mail {
            Ok(())
        } else {
            Err(AppError::new_invalid_argument(
                "EmailAndPass::validate_email invalid email format",
            ))
        }
    }

    fn validate_password(pass: &str, pass_confirm: &str) -> Result<(), AppError> {
        if pass != pass_confirm || pass.len() < MIN_PASSWORD_LENGTH {
            Err(AppError::new_invalid_argument(
                "EmailAndPass::validate_password invalid password",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}
