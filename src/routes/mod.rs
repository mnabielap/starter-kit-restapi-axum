use std::sync::Arc;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    repository::{
        user_repository::{UserRepository, UserRepositoryImpl},
        token_repository::{TokenRepository, TokenRepositoryImpl},
    },
    routes::{
        auth_route::create_auth_router,
        user_route::create_user_router,
    },
    usecase::{
        auth_usecase::{AuthUsecase, AuthUsecaseImpl},
        user_usecase::{UserUsecase, UserUsecaseImpl},
    },
};

mod auth_route;
mod swagger;
mod user_route;

pub fn create_router(db_pool: Arc<sqlx::PgPool>) -> Router {
    // Repositories
    let user_repo: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(db_pool.clone()));
    let token_repo: Arc<dyn TokenRepository> = Arc::new(TokenRepositoryImpl::new(db_pool.clone()));
    
    // Usecases
    let auth_usecase: Arc<dyn AuthUsecase> = Arc::new(AuthUsecaseImpl::new(user_repo.clone(), token_repo.clone()));
    let user_usecase: Arc<dyn UserUsecase> = Arc::new(UserUsecaseImpl::new(user_repo.clone()));

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    
    let v1_routes = Router::new()
        .nest("/auth", create_auth_router(auth_usecase.clone()))
        .nest("/users", create_user_router(user_usecase.clone(), user_repo.clone()));
    
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", swagger::ApiDoc::openapi()))
        .nest("/v1", v1_routes)
        .layer(cors)
}