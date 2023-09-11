use regex::Regex;

use crate::apperror::error::AppError;

#[allow(unused)]
pub fn is_match_mail_format(mail: &str) -> Result<bool, AppError> {
    let reg: Regex = Regex::new(r"^[\w+\-.]+@[a-z\d\-.]+\.[a-z]{2,}$").map_err(|err| {
        AppError::new_internal_with_error(
            "is_match_mail_format failed to create regex for email",
            &err,
        )
    })?;

    Ok(reg.is_match(mail))
}
