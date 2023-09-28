use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateRequest {}

#[derive(Debug, Serialize)]
pub struct CreateResponse {}

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub examples: Vec<DetailResponse>
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub example_id: String,
    pub created_at: i64,
}
