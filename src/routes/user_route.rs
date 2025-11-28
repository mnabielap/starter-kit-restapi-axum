use std::sync::Arc;
use axum::{
    routing::{delete, get, patch, post},
    middleware, Router,
};
use crate::{
    handler::user_handler::*,
    middleware::auth::{admin_only, auth},
    repository::user_repository::UserRepository,
    usecase::user_usecase::UserUsecase,
};

pub fn create_user_router(
    user_usecase: Arc<dyn UserUsecase>,
    user_repo: Arc<dyn UserRepository>,
) -> Router {
    // Routes that can only be accessed by Admin
    let admin_routes = Router::new()
        .route("/", post(create_user_handler).get(get_users_handler))
        .route(
            "/:id",
            patch(update_user_handler).delete(delete_user_handler),
        )
        .route_layer(middleware::from_fn(admin_only));

    // Routes that are accessible to all authenticated users
    let public_routes = Router::new().route("/:id", get(get_user_handler));

    Router::new()
        .merge(admin_routes)
        .merge(public_routes)
        .with_state(user_usecase)
        .route_layer(middleware::from_fn_with_state(user_repo, auth))
}