use axum::{
    Json, http::StatusCode, response::{IntoResponse, Response}
};
use axum_app_config::ServerConfig;
use serde::Serialize;

use crate::handlers::{system_handler, user_handler};

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum Errors {
    // User errors
    UserInvalidPassword,
    UserPasswordNotSet,
    UserInvalidSession,
    UserNotVerified,
    UserNotFound,
    UserUnauthorized,
    UserBanned,
    UserPermissionInsufficient,
    AclDenied(String),
    UserHandleAlreadyExists,
    UserEmailAlreadyExists,
    UserNotBanned,
    UserAlreadyBanned,
    UserDoesNotHaveRole,
    UserAlreadyHasRole,
    CannotManageHigherOrEqualRole,
    UserTokenExpired,
    UserNoRefreshToken,
    UserInvalidToken,

    // System errors
    SysInternalError(String),
    DatabaseError(String),
    TransactionError(String),
    NotFound(String),
    HashingError(String),
    TokenCreationError(String),
}

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        user_handler::log_error(&self);
        system_handler::log_error(&self);

        let (status, code, details) = user_handler::map_response(&self)
            .or_else(|| system_handler::map_response(&self))
            .unwrap_or_else(|| {
                tracing::error!("Unhandled error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "UNKNOWN_ERROR", None)
            });

        // Only include details in dev mode
        let is_dev = ServerConfig::get().is_dev;

        // Error response construction
        let body = ErrorResponse {
            status: status.as_u16(),
            code: code.to_string(),
            details: if is_dev { details } else { None }, // Display detailed information only in the development environment
        };

        (status, Json(body)).into_response()
    }
}

pub async fn handler_404(req: axum::extract::Request) -> impl IntoResponse {
    let path = req.uri().path();
    let method = req.method().to_string();

    Errors::NotFound(format!("Path {} with method {} not found", path, method))
}
