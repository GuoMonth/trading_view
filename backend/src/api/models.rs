use crate::entity::ohlc_data::Model as OhlcDataModel;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

/// 自定义API响应码枚举
#[derive(Debug, Clone, Copy)]
pub enum ApiError {
    // 成功
    Success = 0,
    // 参数错误
    BadRequest = 40001,
    // 未找到资源
    NotFound = 40401,
    // 服务器内部错误
    InternalServerError = 50001,
    // 数据库错误
    DatabaseError = 50002,
    // 时间格式错误
    InvalidDateFormat = 40002,
}

impl ApiError {
    /// 获取HTTP状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Success => StatusCode::OK,
            ApiError::BadRequest | ApiError::InvalidDateFormat => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InternalServerError | ApiError::DatabaseError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// 获取错误信息
    pub fn message(&self) -> &'static str {
        match self {
            ApiError::Success => "Success",
            ApiError::BadRequest => "Bad request",
            ApiError::NotFound => "Resource not found",
            ApiError::InternalServerError => "Internal server error",
            ApiError::DatabaseError => "Database error",
            ApiError::InvalidDateFormat => "Invalid date format",
        }
    }
}

/// 泛型API统一响应对象
#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub message: String,
    /// 响应数据
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: ApiError::Success as i32,
            message: ApiError::Success.message().to_string(),
            data: Some(data),
        }
    }

    /// 创建错误响应
    pub fn error(error: ApiError) -> Self {
        Self {
            code: error as i32,
            message: error.message().to_string(),
            data: None,
        }
    }

    /// 创建带自定义消息的错误响应
    pub fn error_with_message(error: ApiError, message: String) -> Self {
        Self {
            code: error as i32,
            message,
            data: None,
        }
    }
}

/// OHLCR数据按日期范围查询请求参数
#[derive(Deserialize, Debug)]
pub struct OhlcDateRangeRequest {
    pub start: String,
    pub end: String,
}

/// OHLCR数据响应结构
#[derive(Serialize, Debug)]
pub struct OhlcResponse {
    pub data: Vec<OhlcDataModel>,
}
