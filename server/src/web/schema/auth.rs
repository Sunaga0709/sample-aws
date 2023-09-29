use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub birthday: i64,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Clone, Serialize)]
pub struct SignUpResponse {}
