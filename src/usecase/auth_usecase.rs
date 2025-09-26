use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    domain::{
        token_model::{TokenClaims, TokenDetails, TokenResponse},
        user_model::{FilteredUser, Role},
    },
    error::AppError,
    handler::auth_handler::{LoginSchema, RegisterSchema},
    repository::user_repository::UserRepository,
};

fn create_token(user_id: Uuid, secret: &str, expires_in_str: &str) -> Result<TokenDetails, AppError> {
    let expires_in = humantime::parse_duration(expires_in_str)
        .map_err(|_| AppError::InternalServerError)?;
    let now = Utc::now();
    let expires_at = now + Duration::from_std(expires_in).map_err(|_| AppError::InternalServerError)?;

    let claims = TokenClaims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(TokenDetails {
        token,
        expires_in: expires_at.timestamp(),
    })
}

fn create_auth_tokens(user_id: Uuid) -> Result<TokenResponse, AppError> {
    let access_token = create_token(user_id, &CONFIG.jwt_secret, &CONFIG.jwt_access_token_expires_in)?;
    let refresh_token = create_token(user_id, &CONFIG.jwt_secret, &CONFIG.jwt_refresh_token_expires_in)?;
    Ok(TokenResponse {
        access_token,
        refresh_token,
    })
}


#[async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn register(
        &self,
        data: RegisterSchema,
    ) -> Result<(FilteredUser, TokenResponse), AppError>;
    async fn login(&self, data: LoginSchema) -> Result<(FilteredUser, TokenResponse), AppError>;
}


pub struct AuthUsecaseImpl {
    user_repo: Arc<dyn UserRepository>,
}

impl AuthUsecaseImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl AuthUsecase for AuthUsecaseImpl {
    async fn register(
        &self,
        data: RegisterSchema,
    ) -> Result<(FilteredUser, TokenResponse), AppError> {
        if self.user_repo.find_by_email(&data.email).await?.is_some() {
            return Err(AppError::BadRequest("Email already taken".to_string()));
        }

        let password_hash = hash(&data.password, DEFAULT_COST)?;

        let new_user = self
            .user_repo
            .create(&data.name, &data.email, &password_hash, Role::User)
            .await?;

        let tokens = create_auth_tokens(new_user.id)?;

        Ok((new_user.into(), tokens))
    }

    async fn login(&self, data: LoginSchema) -> Result<(FilteredUser, TokenResponse), AppError> {
        let user = self
            .user_repo
            .find_by_email(&data.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        let is_valid = verify(&data.password, &user.password)?;
        if !is_valid {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        let tokens = create_auth_tokens(user.id)?;

        Ok((user.into(), tokens))
    }
}