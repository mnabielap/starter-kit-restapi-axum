use std::sync::Arc;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::{
    repository::user_repository::{UserRepository, UserRepositoryImpl},
    routes::auth_route::create_auth_router
};

mod auth_route;

pub fn create_router(db_pool: Arc<sqlx::PgPool>) -> Router {
    let user_repo: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(db_pool));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let v1_routes = Router::new()
        .nest("/auth", create_auth_router(user_repo.clone()));

    Router::new()
        .nest("/v1", v1_routes)
        .layer(cors)
}