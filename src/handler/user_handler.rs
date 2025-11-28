use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::user_model::{FilteredUser, Role},
    error::{AppError, ErrorResponse},
    repository::user_repository::{PaginatedResult, UserQueryOptions},
    usecase::user_usecase::UserUsecase,
};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserSchema {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUsersQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[utoipa::path(
    post,
    path = "/v1/users",
    tag = "Users",
    request_body = CreateUserSchema,
    responses(
        (status = 201, description = "User created", body = FilteredUser),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_user_handler(
    State(user_usecase): State<Arc<dyn UserUsecase>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, AppError> {
    let new_user = user_usecase
        .create_user(&body.name, &body.email, &body.password, body.role)
        .await?;
    Ok((StatusCode::CREATED, Json(new_user)))
}

#[utoipa::path(
    get,
    path = "/v1/users",
    tag = "Users",
    params(GetUsersQuery),
    responses(
        (status = 200, description = "List of users", body = PaginatedUsers),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_users_handler(
    State(user_usecase): State<Arc<dyn UserUsecase>>,
    Query(params): Query<GetUsersQuery>,
) -> Result<impl IntoResponse, AppError> {
    let options = UserQueryOptions {
        page: params.page,
        limit: params.limit,
    };
    let users = user_usecase.get_users(options).await?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}",
    tag = "Users",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = FilteredUser),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_user_handler(
    State(user_usecase): State<Arc<dyn UserUsecase>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = user_usecase.get_user_by_id(id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    patch,
    path = "/v1/users/{id}",
    tag = "Users",
    params(("id" = Uuid, Path, description = "User ID")),
    request_body = UpdateUserSchema,
    responses(
        (status = 200, description = "User updated", body = FilteredUser),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_user_handler(
    State(user_usecase): State<Arc<dyn UserUsecase>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, AppError> {
    let updated_user = user_usecase
        .update_user(id, body.name, body.email, body.password)
        .await?;
    Ok(Json(updated_user))
}

#[utoipa::path(
    delete,
    path = "/v1/users/{id}",
    tag = "Users",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_user_handler(
    State(user_usecase): State<Arc<dyn UserUsecase>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    user_usecase.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}