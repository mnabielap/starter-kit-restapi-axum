use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use crate::{
    config::CONFIG,
    domain::{
        token_model::TokenClaims,
        user_model::{Role, User},
    },
    error::AppError,
    repository::user_repository::UserRepository,
};

pub async fn auth(
    State(user_repo): State<Arc<dyn UserRepository>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "));

    let token = token.ok_or_else(|| AppError::Unauthorized("You are not logged in".to_string()))?;

    let claims = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?
    .claims;

    if claims.token_type != "access" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    let user = user_repo.find_by_id(claims.sub).await?.ok_or_else(|| {
        AppError::NotFound("The user belonging to this token no longer exists".to_string())
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn admin_only(req: Request, next: Next) -> Result<Response, AppError> {
    let user = req.extensions().get::<User>().ok_or_else(|| {
        AppError::InternalServerError
    })?;

    if user.role >= Role::Admin {
        Ok(next.run(req).await)
    } else {
        Err(AppError::Forbidden(
            "You do not have permission to access this resource".to_string(),
        ))
    }
}