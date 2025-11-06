use axum::response::{IntoResponse, Response};
use axum::Json;
use anyhow::Error;
use tracing::error;

use crate::api::models::{ApiResponse, ApiError};

pub struct ApiErrorWrapper(Error);

impl From<Error> for ApiErrorWrapper {
    fn from(err: Error) -> Self {
        ApiErrorWrapper(err)
    }
}

impl IntoResponse for ApiErrorWrapper {
    fn into_response(self) -> Response {
        error!("API Error: {}", self.0);
        
        // 根据错误类型确定合适的ApiError
        let api_error = if self.0.downcast_ref::<chrono::ParseError>().is_some() {
            ApiError::InvalidDateFormat
        } else if self.0.downcast_ref::<sea_orm::DbErr>().is_some() {
            ApiError::DatabaseError
        } else {
            ApiError::InternalServerError
        };
        
        let response = ApiResponse::<()>::error_with_message(api_error, self.0.to_string());
        (api_error.status_code(), Json(response)).into_response()
    }
}