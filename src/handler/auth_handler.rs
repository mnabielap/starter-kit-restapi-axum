use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    domain::{token_model::TokenResponse, user_model::FilteredUser},
    error::AppError,
    usecase::auth_usecase::AuthUsecase,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginSchema {
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    user: FilteredUser,
    tokens: TokenResponse,
}

pub async fn register_handler(
    State(auth_usecase): State<Arc<dyn AuthUsecase>>,
    Json(body): Json<RegisterSchema>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;

    let (user, tokens) = auth_usecase.register(body).await?;

    let response = AuthResponse { user, tokens };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login_handler(
    State(auth_usecase): State<Arc<dyn AuthUsecase>>,
    Json(body): Json<LoginSchema>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;
    
    let (user, tokens) = auth_usecase.login(body).await?;

    let response = AuthResponse { user, tokens };

    Ok((StatusCode::OK, Json(response)))
}