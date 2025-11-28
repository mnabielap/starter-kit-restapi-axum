use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{domain::token_model::TokenType, error::AppError};

#[derive(Debug, sqlx::FromRow)]
pub struct Token {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub token_type: TokenType,
    pub expires_at: DateTime<Utc>,
    pub blacklisted: bool,
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn create(&self, token: &str, user_id: Uuid, expires_at: DateTime<Utc>, token_type: TokenType) -> Result<Token, AppError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<Token>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn delete_user_tokens_by_type(&self, user_id: Uuid, token_type: TokenType) -> Result<(), AppError>;
}

pub struct TokenRepositoryImpl {
    db_pool: Arc<PgPool>,
}

impl TokenRepositoryImpl {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn create(&self, token: &str, user_id: Uuid, expires_at: DateTime<Utc>, token_type: TokenType) -> Result<Token, AppError> {
        sqlx::query_as!(
            Token,
            r#"
            INSERT INTO tokens (token, user_id, expires_at, token_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, token, user_id, token_type AS "token_type!: TokenType", expires_at, blacklisted
            "#,
            token, user_id, expires_at, token_type as TokenType
        )
        .fetch_one(&*self.db_pool)
        .await
        .map_err(Into::into)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<Token>, AppError> {
        sqlx::query_as!(
            Token,
            r#"
            SELECT id, token, user_id, token_type AS "token_type!: TokenType", expires_at, blacklisted
            FROM tokens WHERE token = $1 AND blacklisted = false
            "#,
            token
        )
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(Into::into)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM tokens WHERE id = $1")
            .bind(id)
            .execute(&*self.db_pool)
            .await?;
        Ok(())
    }

    async fn delete_user_tokens_by_type(&self, user_id: Uuid, token_type: TokenType) -> Result<(), AppError> {
        sqlx::query("DELETE FROM tokens WHERE user_id = $1 AND token_type = $2")
            .bind(user_id)
            .bind(token_type)
            .execute(&*self.db_pool)
            .await?;
        Ok(())
    }
}