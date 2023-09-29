use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::apperror::error::AppError;
use crate::gateway::aws_s3::AwsS3;
use crate::usecase::image::ImageUsecase;
use crate::web::schema::image::{UploadImageRequest, UploadImageResponse};

type ImageUsecaseState = State<ImageUsecase<AwsS3>>;

pub async fn upload_image(
    State(uc): ImageUsecaseState,
    Json(param): Json<UploadImageRequest>,
) -> Result<impl IntoResponse, AppError> {
    match uc.upload_image(&param.name, &param.image).await {
        Ok(_) => Ok(Json(UploadImageResponse {})),
        Err(err) => Err(err),
    }
}
