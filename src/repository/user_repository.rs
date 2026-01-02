use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool, QueryBuilder, Row};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    domain::user_model::{Role, User},
    error::AppError,
};

#[derive(Debug, PartialEq)]
pub enum UserSortDirection {
    Asc,
    Desc,
}

#[derive(Debug)]
pub struct UserQueryOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub scope: Option<String>, // "all", "name", "email", "id"
    pub role: Option<Role>,
    pub sort_column: String,
    pub sort_direction: UserSortDirection,
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
    db_pool: Arc<SqlitePool>,
}

impl UserRepositoryImpl {
    pub fn new(db_pool: Arc<SqlitePool>) -> Self {
        Self { db_pool }
    }

    // Helper to build the WHERE clause based on options
    fn apply_filters<'a>(&self, query: &mut QueryBuilder<'a, Sqlite>, options: &'a UserQueryOptions) {
        query.push(" WHERE 1=1 ");

        if let Some(role) = &options.role {
            query.push(" AND role = ");
            query.push_bind(role.clone() as Role);
        }

        if let Some(search) = &options.search {
            if !search.is_empty() {
                let scope = options.scope.as_deref().unwrap_or("all");
                let like_pattern = format!("%{}%", search);

                match scope {
                    "name" => {
                        query.push(" AND name LIKE ");
                        query.push_bind(like_pattern);
                    }
                    "email" => {
                        query.push(" AND email LIKE ");
                        query.push_bind(like_pattern);
                    }
                    "id" => {
                        if let Ok(uuid) = Uuid::parse_str(search) {
                            query.push(" AND id = ");
                            query.push_bind(uuid);
                        } else {
                            query.push(" AND 0=1 ");
                        }
                    }
                    _ => {
                        // Scope 'all'
                        query.push(" AND (name LIKE ");
                        query.push_bind(like_pattern.clone());
                        query.push(" OR email LIKE ");
                        query.push_bind(like_pattern);
                        
                        if let Ok(uuid) = Uuid::parse_str(search) {
                            query.push(" OR id = ");
                            query.push_bind(uuid);
                        }
                        query.push(")");
                    }
                }
            }
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            r#"SELECT 
                id as "id!: Uuid", 
                name, 
                email, 
                password, 
                role AS "role!: Role", 
                is_email_verified, 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>" 
            FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(Into::into)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
             r#"SELECT 
                id as "id!: Uuid", 
                name, 
                email, 
                password, 
                role AS "role!: Role", 
                is_email_verified, 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>" 
            FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&*self.db_pool)
        .await?;
        Ok(user)
    }

    async fn create(&self, name: &str, email: &str, password_hash: &str, role: Role) -> Result<User, AppError> {
        let new_id = Uuid::new_v4();
        let role_val = role as Role;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, name, email, password, role)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING 
                id as "id!: Uuid", 
                name, 
                email, 
                password, 
                role AS "role!: Role", 
                is_email_verified, 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            new_id, name, email, password_hash, role_val
        )
        .fetch_one(&*self.db_pool)
        .await?;
        Ok(user)
    }

    async fn query_users(&self, options: UserQueryOptions) -> Result<PaginatedResult<User>, AppError> {
        let page = options.page.unwrap_or(1).max(1);
        let limit = options.limit.unwrap_or(10).max(1);
        let offset = (page - 1) * limit;

        // 1. Build Count Query
        let mut count_query_builder = QueryBuilder::new("SELECT COUNT(*) FROM users");
        self.apply_filters(&mut count_query_builder, &options);
        
        let total_results: i64 = count_query_builder.build_query_scalar()
            .fetch_one(&*self.db_pool)
            .await
            .unwrap_or(0);

        // 2. Build Data Query
        let mut query_builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new(
            r#"SELECT 
                id, 
                name, 
                email, 
                password, 
                role, 
                is_email_verified, 
                created_at, 
                updated_at 
            FROM users"#
        );
        self.apply_filters(&mut query_builder, &options);

        // Sorting
        let sort_column = match options.sort_column.as_str() {
            "name" => "name",
            "email" => "email",
            "role" => "role",
            "id" => "id",
            _ => "created_at", // Default safety
        };
        
        let direction = match options.sort_direction {
            UserSortDirection::Asc => "ASC",
            UserSortDirection::Desc => "DESC",
        };

        query_builder.push(format!(" ORDER BY {} {} ", sort_column, direction));

        // Pagination
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        let users = query_builder.build_query_as::<User>()
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
        let role_val = user.role.clone() as Role;

        sqlx::query_as!(
            User,
            r#"
            UPDATE users SET name = $1, email = $2, password = $3, role = $4, is_email_verified = $5, updated_at = CURRENT_TIMESTAMP
            WHERE id = $6
            RETURNING 
                id as "id!: Uuid", 
                name, 
                email, 
                password, 
                role AS "role!: Role", 
                is_email_verified, 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            user.name, user.email, user.password, role_val, user.is_email_verified, user.id
        )
        .fetch_one(&*self.db_pool)
        .await
        .map_err(Into::into)
    }
}