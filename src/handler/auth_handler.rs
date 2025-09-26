use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;
use utoipa::{path, ToSchema};

use crate::{
    domain::{token_model::TokenResponse, user_model::FilteredUser},
    error::{AppError, ErrorResponse},
    usecase::auth_usecase::AuthUsecase,
};

// --- Schemas for Request Bodies ---

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginSchema {
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenSchema {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

// --- Schema for Response Body ---

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub user: FilteredUser,
    pub tokens: TokenResponse,
}

// --- Handler Functions ---

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    tag = "Auth",
    request_body = RegisterSchema,
    responses(
        (status = 201, description = "User created successfully", body = AuthResponse),
        (status = 400, description = "Invalid input or email already taken", body = ErrorResponse)
    )
)]
pub async fn register_handler(
    State(auth_usecase): State<Arc<dyn AuthUsecase>>,
    Json(body): Json<RegisterSchema>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;
    let (user, tokens) = auth_usecase.register(body).await?;
    let response = AuthResponse { user, tokens };
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "Auth",
    request_body = LoginSchema,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid email or password", body = ErrorResponse)
    )
)]
pub async fn login_handler(
    State(auth_usecase): State<Arc<dyn AuthUsecase>>,
    Json(body): Json<LoginSchema>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;
    let (user, tokens) = auth_usecase.login(body).await?;
    let response = AuthResponse { user, tokens };
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tag = "Auth",
    request_body = RefreshTokenSchema,
    responses(
        (status = 204, description = "Logout successful"),
        (status = 404, description = "Token not found", body = ErrorResponse)
    )
)]
pub async fn logout_handler(
    State(auth_usecase): State<Arc<dyn AuthUsecase>>,
    Json(body): Json<RefreshTokenSchema>,
) -> Result<impl IntoResponse, AppError> {
    auth_usecase.logout(body.refresh_token).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh-tokens",
    tag = "Auth",
    request_body = RefreshTokenSchema,
    responses(
        (status = 200, description = "Tokens refreshed successfully", body = TokenResponse),
        (status = 401, description = "Unauthorized or invalid refresh token", body = ErrorResponse)
    )
)]
pub async fn refresh_tokens_handler(
    State(auth_usecase): State<Arc<dyn AuthUsecase>>,
    Json(body): Json<RefreshTokenSchema>,
) -> Result<impl IntoResponse, AppError> {
    let tokens = auth_usecase.refresh_auth(body.refresh_token).await?;
    Ok((StatusCode::OK, Json(tokens)))
}