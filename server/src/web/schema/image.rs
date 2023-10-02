use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, ToSchema)]
pub struct UploadImageRequest {
    pub name: String,
    pub image: String,
}

#[derive(Clone, Serialize, ToSchema)]
pub struct UploadImageResponse {}
