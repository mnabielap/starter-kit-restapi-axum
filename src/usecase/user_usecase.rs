use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use bcrypt::hash;

use crate::{
    domain::user_model::{FilteredUser, Role, User},
    error::AppError,
    repository::user_repository::{PaginatedResult, UserRepository, UserQueryOptions},
};

#[async_trait]
pub trait UserUsecase: Send + Sync {
    async fn create_user(&self, name: &str, email: &str, password: &str, role: Role) -> Result<FilteredUser, AppError>;
    async fn get_users(&self, options: UserQueryOptions) -> Result<PaginatedResult<FilteredUser>, AppError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<FilteredUser, AppError>;
    async fn update_user(&self, id: Uuid, name: Option<String>, email: Option<String>, password: Option<String>) -> Result<FilteredUser, AppError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), AppError>;
}

pub struct UserUsecaseImpl {
    user_repo: Arc<dyn UserRepository>,
}

impl UserUsecaseImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserUsecase for UserUsecaseImpl {
    async fn create_user(&self, name: &str, email: &str, password: &str, role: Role) -> Result<FilteredUser, AppError> {
        if self.user_repo.find_by_email(email).await?.is_some() {
            return Err(AppError::BadRequest("Email already taken".to_string()));
        }
        let password_hash = hash(password, 10)?;
        let new_user = self.user_repo.create(name, email, &password_hash, role).await?;
        Ok(new_user.into())
    }

    async fn get_users(&self, options: UserQueryOptions) -> Result<PaginatedResult<FilteredUser>, AppError> {
        let paginated_users = self.user_repo.query_users(options).await?;
        Ok(PaginatedResult {
            results: paginated_users.results.into_iter().map(|u| u.into()).collect(),
            page: paginated_users.page,
            limit: paginated_users.limit,
            total_pages: paginated_users.total_pages,
            total_results: paginated_users.total_results,
        })
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<FilteredUser, AppError> {
        let user = self.user_repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        Ok(user.into())
    }

    async fn update_user(&self, id: Uuid, name: Option<String>, email: Option<String>, password: Option<String>) -> Result<FilteredUser, AppError> {
        let password_hash = if let Some(p) = password { Some(hash(p, 10)?) } else { None };
        let updated_user = self.user_repo.update_by_id(id, name, email, password_hash).await?;
        Ok(updated_user.into())
    }
    
    async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        self.user_repo.delete_by_id(id).await
    }
}