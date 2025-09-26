use std::sync::Arc;
use axum::{routing::post, Router};

use crate::{
    handler::auth_handler::{login_handler, register_handler},
    usecase::auth_usecase::AuthUsecase,
};

pub fn create_auth_router(auth_usecase: Arc<dyn AuthUsecase>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(auth_usecase)
}