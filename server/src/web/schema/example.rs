use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRequest {}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResponse {}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetResponse {
    pub examples: Vec<DetailResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetailResponse {
    pub example_id: String,
    pub created_at: i64,
}
