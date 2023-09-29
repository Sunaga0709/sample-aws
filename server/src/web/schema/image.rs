use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct UploadImageRequest {
    pub name: String,
    pub image: String,
}

#[derive(Clone, Serialize)]
pub struct UploadImageResponse {}
