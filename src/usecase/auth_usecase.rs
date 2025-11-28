use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Validation};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    domain::{
        token_model::{TokenClaims, TokenDetails, TokenResponse, TokenType},
        user_model::{FilteredUser, Role, User},
    },
    error::AppError,
    handler::auth_handler::{LoginSchema, RegisterSchema},
    repository::{
        token_repository::{Token, TokenRepository},
        user_repository::UserRepository,
    },
};

fn create_token(user_id: Uuid, secret: &str, expires_in_str: &str, token_type: &str) -> Result<TokenDetails, AppError> {
    let expires_in = humantime::parse_duration(expires_in_str).map_err(|_| AppError::InternalServerError)?;
    let now = Utc::now();
    let expires_at = now + Duration::from_std(expires_in).map_err(|_| AppError::InternalServerError)?;
    
    let claims = TokenClaims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
        token_type: token_type.to_string(),
    };

    let token = encode(&jsonwebtoken::Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(TokenDetails { token, expires_in: expires_at.timestamp() })
}

fn create_auth_tokens(user_id: Uuid) -> Result<TokenResponse, AppError> {
    let access_token = create_token(user_id, &CONFIG.jwt_secret, &CONFIG.jwt_access_token_expires_in, "access")?;
    let refresh_token = create_token(user_id, &CONFIG.jwt_secret, &CONFIG.jwt_refresh_token_expires_in, "refresh")?;
    Ok(TokenResponse { access_token, refresh_token })
}


#[async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn register(&self, data: RegisterSchema) -> Result<(FilteredUser, TokenResponse), AppError>;
    async fn login(&self, data: LoginSchema) -> Result<(FilteredUser, TokenResponse), AppError>;
    async fn logout(&self, refresh_token: String) -> Result<(), AppError>;
    async fn refresh_auth(&self, refresh_token: String) -> Result<TokenResponse, AppError>;
}

pub struct AuthUsecaseImpl {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn TokenRepository>,
}

impl AuthUsecaseImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>, token_repo: Arc<dyn TokenRepository>) -> Self {
        Self { user_repo, token_repo }
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
        let new_user = self.user_repo.create(&data.name, &data.email, &password_hash, Role::User).await?;
        let tokens = create_auth_tokens(new_user.id)?;
        
        let expires_at = Utc::now() + Duration::days(30);
        self.token_repo.create(&tokens.refresh_token.token, new_user.id, expires_at, TokenType::Refresh).await?;

        Ok((new_user.into(), tokens))
    }

    async fn login(&self, data: LoginSchema) -> Result<(FilteredUser, TokenResponse), AppError> {
        let user = self.user_repo.find_by_email(&data.email).await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;
        let is_valid = verify(&data.password, &user.password)?;
        if !is_valid {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }
        let tokens = create_auth_tokens(user.id)?;
        
        let expires_at = Utc::now() + Duration::days(30);
        self.token_repo.create(&tokens.refresh_token.token, user.id, expires_at, TokenType::Refresh).await?;

        Ok((user.into(), tokens))
    }

    async fn logout(&self, refresh_token: String) -> Result<(), AppError> {
        let token_doc = self.token_repo.find_by_token(&refresh_token).await?
            .ok_or_else(|| AppError::NotFound("Token not found".to_string()))?;
        self.token_repo.delete(token_doc.id).await
    }
    
    async fn refresh_auth(&self, refresh_token: String) -> Result<TokenResponse, AppError> {
        let token_doc = self.token_repo.find_by_token(&refresh_token).await?
            .ok_or_else(|| AppError::Unauthorized("Please authenticate".to_string()))?;

        let user = self.user_repo.find_by_id(token_doc.user_id).await?
            .ok_or_else(|| AppError::Unauthorized("Please authenticate".to_string()))?;
            
        self.token_repo.delete(token_doc.id).await?;

        let tokens = create_auth_tokens(user.id)?;
        let expires_at = Utc::now() + Duration::days(30);
        self.token_repo.create(&tokens.refresh_token.token, user.id, expires_at, TokenType::Refresh).await?;
        
        Ok(tokens)
    }
}