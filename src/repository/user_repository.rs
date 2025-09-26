use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    domain::user_model::{User, Role},
    error::AppError,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create(&self, name: &str, email: &str, password_hash: &str, role: Role) -> Result<User, AppError>;
}

pub struct UserRepositoryImpl {
    db_pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, name, email, password, role AS "role!: Role", is_email_verified, created_at, updated_at FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&*self.db_pool)
        .await?;
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
             r#"SELECT id, name, email, password, role AS "role!: Role", is_email_verified, created_at, updated_at FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&*self.db_pool)
        .await?;
        Ok(user)
    }

    async fn create(&self, name: &str, email: &str, password_hash: &str, role: Role) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, password, role AS "role!: Role", is_email_verified, created_at, updated_at
            "#,
            name, email, password_hash, role as Role
        )
        .fetch_one(&*self.db_pool)
        .await?;
        Ok(user)
    }
}