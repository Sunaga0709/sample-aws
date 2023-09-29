use axum::http::header::HeaderValue;
use cookie::Cookie;
use std::collections::HashMap;

use crate::apperror::error::AppError;

pub const SESSION_KEY: &str = "session";
pub const ACCESS_TOKEN_KEY: &str = "access_token";
pub const REFRESH_TOKEN_KEY: &str = "refresh_token";

pub fn header_values(value: HashMap<&str, &str>) -> Result<Vec<HeaderValue>, AppError> {
    let mut head_vec: Vec<HeaderValue> = Vec::new();
    for (key, value) in value {
        let cookie_value: Cookie = Cookie::build(key, value)
            .path("/")
            .secure(false)
            .http_only(true)
            .finish();

        let head_value: HeaderValue = HeaderValue::from_str(&cookie_value.to_string()).map_err(
            |err| {
                AppError::new_internal_with_error(
                    "AxumResponse::set_cookie_for_axum failed to convert header value from cookie string",
                    &err
                )
            }
        )?;

        head_vec.push(head_value);
    }

    Ok(head_vec)
}
