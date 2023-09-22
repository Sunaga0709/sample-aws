use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use sqlx::FromRow;

use super::datetime;
use super::uuid;
use crate::apperror::error::AppError;

#[derive(Clone, Debug, Eq, FromRow, PartialEq)]
pub struct Image {
    pub image_id: String,
    pub name: String,
    pub decode_data: Vec<u8>,
    pub created_at: i64,
}

impl Image {
    pub fn new(name: &str, data: &str) -> Result<Self, AppError> {
        Self::validate_name(name)?;
        Self::validate_image_data(data)?;

        let decode_data: Vec<u8> = Self::decode_base64(data)?;

        Ok(Self {
            image_id: uuid::new_uuid(),
            name: name.to_owned(),
            decode_data,
            created_at: datetime::now_unix(),
        })
    }

    fn validate_name(name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            Err(AppError::new_invalid_argument(
                "Image::validate_name name is empty",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_image_data(data: &str) -> Result<(), AppError> {
        if data.is_empty() {
            Err(AppError::new_invalid_argument(
                "Image::validate_image_data data is empty",
            ))
        } else {
            Ok(())
        }
    }

    fn decode_base64(data: &str) -> Result<Vec<u8>, AppError> {
        let decoded_data: Vec<u8> =
            engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD)
                .decode(data)
                .map_err(|err| {
                    AppError::new_internal_with_error(
                        "Image::decode_base64 failed to decode base64 image data",
                        &err,
                    )
                })?;

        Ok(decoded_data)
    }
}
