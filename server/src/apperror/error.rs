use std::error::Error as StdError;
use tonic::{Code, Status};

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
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Status {
        match err {
            AppError::InvalidArgument(msg) => Status::new(Code::InvalidArgument, msg),
            AppError::NotFound(msg) => Status::new(Code::NotFound, msg),
            AppError::AlreadyExists(msg) => Status::new(Code::AlreadyExists, msg),
            AppError::Internal(msg) => Status::new(Code::Internal, msg),
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
