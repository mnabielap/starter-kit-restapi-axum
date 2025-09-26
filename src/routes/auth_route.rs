use std::sync::Arc;
use axum::{
    routing::post,
    Router,
};
use crate::{
    handler::auth_handler::{register_handler, login_handler},
    repository::user_repository::UserRepository,
};

pub fn create_auth_router(user_repo: Arc<dyn UserRepository>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(user_repo)
}