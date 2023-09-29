use axum::http::StatusCode;
use axum::response::{self, IntoResponse, Response as AxumResponse};
use std::error::Error as StdError;
use tonic::Code::{
    AlreadyExists as GrpcAlreadyExists, Internal as GrpcInternal,
    InvalidArgument as GrpcInvalidArgument, NotFound as GrpcNotFound,
};
use tonic::Status as TonicStatus;

use crate::web::schema::error_response::ErrorResponse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppError {
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    Internal(String),
}

impl AppError {
    pub fn new_invalid_argument(msg: &str) -> Self {
        AppError::InvalidArgument(msg.to_string())
    }

    pub fn new_not_found(msg: &str) -> Self {
        AppError::NotFound(msg.to_string())
    }

    pub fn new_already_exists(msg: &str) -> Self {
        AppError::AlreadyExists(msg.to_string())
    }

    pub fn new_internal(msg: &str) -> Self {
        AppError::Internal(msg.to_string())
    }

    pub fn new_invalid_argument_with_error(msg: &str, err: &dyn StdError) -> Self {
        AppError::InvalidArgument(format!("{}/ {}", msg, err))
    }

    pub fn new_not_found_with_error(msg: &str, err: &dyn StdError) -> Self {
        AppError::NotFound(format!("{}/ {}", msg, err))
    }

    pub fn new_already_exists_with_error(msg: &str, err: &dyn StdError) -> Self {
        AppError::AlreadyExists(format!("{}/ {}", msg, err))
    }

    pub fn new_internal_with_error(msg: &str, err: &dyn StdError) -> Self {
        AppError::Internal(format!("{}/ {}", msg, err))
    }

    fn value(&self) -> String {
        match self {
            Self::InvalidArgument(msg) => msg.to_owned(),
            Self::NotFound(msg) => msg.to_owned(),
            Self::AlreadyExists(msg) => msg.to_owned(),
            Self::Internal(msg) => msg.to_owned(),
        }
    }

    fn status_code(&self) -> u16 {
        match self {
            Self::InvalidArgument(_) => 400_u16,
            Self::NotFound(_) => 404_u16,
            Self::AlreadyExists(_) => 409_u16,
            _ => 500_u16,
        }
    }

    fn reason(&self) -> String {
        match self {
            Self::InvalidArgument(_) => String::from("BadRequest"),
            Self::NotFound(_) => String::from("NotFound"),
            Self::AlreadyExists(_) => String::from("Conflict"),
            _ => String::from("Internal"),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::InvalidArgument(_) => String::from("不正な値が入力されました。"),
            Self::NotFound(_) => String::from("リソースが見つかりません。"),
            Self::AlreadyExists(_) => String::from("既に存在します。"),
            _ => String::from("サーバーエラーが発生しました。"),
        }
    }

    fn to_status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AppError> for TonicStatus {
    fn from(err: AppError) -> TonicStatus {
        match err {
            AppError::InvalidArgument(msg) => TonicStatus::new(GrpcInvalidArgument, msg),
            AppError::NotFound(msg) => TonicStatus::new(GrpcNotFound, msg),
            AppError::AlreadyExists(msg) => TonicStatus::new(GrpcAlreadyExists, msg),
            AppError::Internal(msg) => TonicStatus::new(GrpcInternal, msg),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::InvalidArgument(ref message) => write!(f, "Invalid argument: {}", message),
            AppError::NotFound(ref message) => write!(f, "Not found: {}", message),
            AppError::AlreadyExists(ref message) => write!(f, "Already exists: {}", message),
            AppError::Internal(ref message) => write!(f, "Internal error: {}", message),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> AxumResponse {
        let err_res: ErrorResponse = ErrorResponse {
            code: self.status_code(),
            reason: self.reason(),
            message: self.message(),
        };

        let mut res: AxumResponse = response::Json(err_res).into_response();
        *res.status_mut() = self.to_status_code();
        res.extensions_mut().insert(self);

        res
    }
}
