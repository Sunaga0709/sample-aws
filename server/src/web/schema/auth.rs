use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, ToSchema)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub birthday: i64,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Clone, Serialize, ToSchema)]
pub struct SignUpResponse {}
