use std::sync::Arc;
use axum::{routing::post, Router};

use crate::{
    handler::auth_handler::{login_handler, register_handler, logout_handler, refresh_tokens_handler},
    usecase::auth_usecase::AuthUsecase,
};

pub fn create_auth_router(auth_usecase: Arc<dyn AuthUsecase>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/refresh-tokens", post(refresh_tokens_handler))
        .with_state(auth_usecase)
}