use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    domain::user_model::{Role, User},
    error::AppError,
};

#[derive(Debug, Deserialize)]
pub struct UserQueryOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult<T: for<'a> ToSchema<'a> + Serialize> {
    pub results: Vec<T>,
    pub page: u32,
    pub limit: u32,
    #[schema(example = 10)]
    pub total_pages: u32,
    #[schema(example = 100)]
    pub total_results: i64,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create(&self, name: &str, email: &str, password_hash: &str, role: Role) -> Result<User, AppError>;
    async fn query_users(&self, options: UserQueryOptions) -> Result<PaginatedResult<User>, AppError>;
    async fn update_by_id(&self, id: Uuid, name: Option<String>, email: Option<String>, password: Option<String>) -> Result<User, AppError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<(), AppError>;
    async fn save(&self, user: &User) -> Result<User, AppError>;
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
        sqlx::query_as!(
            User,
            r#"SELECT id, name, email, password, role AS "role!: Role", is_email_verified, created_at, updated_at FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(Into::into)
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

    async fn query_users(&self, options: UserQueryOptions) -> Result<PaginatedResult<User>, AppError> {
        let page = options.page.unwrap_or(1);
        let limit = options.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        let total_results: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&*self.db_pool).await?;
        
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, password, role AS "role!: Role", is_email_verified, created_at, updated_at
            FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&*self.db_pool)
        .await?;
        
        let total_pages = if total_results > 0 {
            (total_results as f64 / limit as f64).ceil() as u32
        } else {
            0
        };

        Ok(PaginatedResult {
            results: users,
            page,
            limit,
            total_pages,
            total_results,
        })
    }

    async fn update_by_id(&self, id: Uuid, name: Option<String>, email: Option<String>, password: Option<String>) -> Result<User, AppError> {
        let mut user = self.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        if let Some(name) = name { user.name = name; }
        if let Some(email) = email { user.email = email; }
        if let Some(password) = password { user.password = password; }
        self.save(&user).await
    }
    
    async fn delete_by_id(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&*self.db_pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }
        Ok(())
    }

    async fn save(&self, user: &User) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users SET name = $1, email = $2, password = $3, role = $4, is_email_verified = $5, updated_at = NOW()
            WHERE id = $6
            RETURNING id, name, email, password, role AS "role!: Role", is_email_verified, created_at, updated_at
            "#,
            user.name, user.email, user.password, user.role.clone() as Role, user.is_email_verified, user.id
        )
        .fetch_one(&*self.db_pool)
        .await
        .map_err(Into::into)
    }
}