use tracing::{debug, info, warn, error};

use axum::{
    response::{IntoResponse, Response, Json},
    http::StatusCode,
};

use serde::Serialize;
use std::{fmt, result};

pub type Result<T> = result::Result<T, ServiceError>;
pub type JsonResult<T> = result::Result<Json<T>, ServiceError>;
pub type FileStreamResult = result::Result<axum::response::Response, ServiceError>;

// JSON Response error DTO (what actual user sees)
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

// Internal API error
// Used only in internal functions
#[derive(Debug)]
pub enum ServiceError {
    FileNotFound(String),
    SeasonNotFound(String),
    Parse(String),
    Io(String),
}

impl std::error::Error for ServiceError {}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> Self {
        ServiceError::Io(error.to_string())
    }
}

impl From<toml::de::Error> for ServiceError {
    fn from(error: toml::de::Error) -> Self {
        ServiceError::Parse(error.to_string())
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound(p) => write!(f, "File not found at path: {}", p),
            Self::SeasonNotFound(s) => write!(f, "Season '{}' not found", s),
            Self::Parse(e) => write!(f, "Failed to parse config or manifest: {}", e),
            Self::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        AppError::from(self).into_response()
    }
}

// HTTP Error for Axum
// Used only in route handlers
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                msg,
            ),

            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST",
                msg,
            ),

            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "An internal server error occurred. Please try again later.".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error_code: error_code.to_string(),
            message
        });

        (status, body).into_response()
    }
}


// Automatic conversion ServiceError -> AppError
// (Internal logic error into public API error)
impl From<ServiceError> for AppError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::FileNotFound(path) => {
                error!("Internal missing file: {}", path);
                AppError::NotFound("Requested resource file is missing".to_string())
            }

            ServiceError::SeasonNotFound(season) => {
                AppError::NotFound(format!("Season '{}' was not found on server", season))
            }

            ServiceError::Parse(details) => {
                error!("Config parse failed: {}", details);
                AppError::InternalServerError
            }

            ServiceError::Io(io_error) => {
                error!("Unhandled I/O error: {}", io_error);
                AppError::InternalServerError
            }
        }
    }
}
