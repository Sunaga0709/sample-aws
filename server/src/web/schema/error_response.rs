use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: u16,       // e.g. 404
    pub reason: String,  // e.g. "NotFound"
    pub message: String, // e.g. "リソースが存在しません。"
}
