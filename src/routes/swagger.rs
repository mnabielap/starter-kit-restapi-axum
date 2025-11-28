use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use crate::{
    domain::{
        token_model::{TokenDetails, TokenResponse},
        user_model::FilteredUser,
    },
    error::ErrorResponse,
    handler::{
        auth_handler::{self, AuthResponse, LoginSchema, RefreshTokenSchema, RegisterSchema},
        user_handler::{self, CreateUserSchema, UpdateUserSchema},
    },
    repository::user_repository::PaginatedResult,
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}


#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handler::register_handler,
        auth_handler::login_handler,
        auth_handler::logout_handler,
        auth_handler::refresh_tokens_handler,
        user_handler::create_user_handler,
        user_handler::get_users_handler,
        user_handler::get_user_handler,
        user_handler::update_user_handler,
        user_handler::delete_user_handler,
    ),
    components(
        schemas(
            // Schemas
            AuthResponse, LoginSchema, RefreshTokenSchema, RegisterSchema,
            CreateUserSchema, UpdateUserSchema,
            FilteredUser,
            TokenResponse, TokenDetails,
            ErrorResponse,
            PaginatedResult<FilteredUser>,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;