use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    config::CONFIG,
    domain::{
        token_model::{TokenClaims, TokenDetails, TokenResponse},
        user_model::{FilteredUser, Role},
    },
    error::AppError,
    repository::user_repository::UserRepository,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;

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

fn create_token(user_id: Uuid, secret: &str, expires_in_str: &str) -> Result<TokenDetails, AppError> {
    let expires_in = humantime::parse_duration(expires_in_str).unwrap();
    let now = Utc::now();
    let expires_at = now + Duration::from_std(expires_in).unwrap();
    let claims = TokenClaims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(TokenDetails { token, expires_in: expires_at.timestamp() })
}

fn create_auth_tokens(user_id: Uuid) -> Result<TokenResponse, AppError> {
    let access_token = create_token(user_id, &CONFIG.jwt_secret, &CONFIG.jwt_access_token_expires_in)?;
    let refresh_token = create_token(user_id, &CONFIG.jwt_secret, &CONFIG.jwt_refresh_token_expires_in)?;
    Ok(TokenResponse { access_token, refresh_token })
}


pub async fn register_handler(
    State(user_repo): State<Arc<dyn UserRepository>>,
    Json(body): Json<RegisterSchema>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;

    if user_repo.find_by_email(&body.email).await?.is_some() {
        return Err(AppError::BadRequest("Email already taken".to_string()));
    }

    let password_hash = hash(&body.password, DEFAULT_COST)?;
    let new_user = user_repo
        .create(&body.name, &body.email, &password_hash, Role::User)
        .await?;

    let tokens = create_auth_tokens(new_user.id)?;
    let response = AuthResponse {
        user: new_user.into(),
        tokens,
    };

    Ok((StatusCode::CREATED, Json(response)))
}


pub async fn login_handler(
    State(user_repo): State<Arc<dyn UserRepository>>,
    Json(body): Json<LoginSchema>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()?;

    let user = user_repo
        .find_by_email(&body.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    let is_valid = verify(&body.password, &user.password)?;
    if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    let tokens = create_auth_tokens(user.id)?;
    let response = AuthResponse {
        user: user.into(),
        tokens,
    };

    Ok((StatusCode::OK, Json(response)))
}