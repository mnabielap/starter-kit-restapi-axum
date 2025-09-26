use utoipa::{OpenApi};
use crate::{
    handler::auth_handler::{self, AuthResponse, RegisterSchema, LoginSchema},
    domain::{user_model::{FilteredUser, Role}, token_model::{TokenResponse, TokenDetails}},
    error::ErrorResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handler::register_handler,
        auth_handler::login_handler,
    ),
    components(
        schemas(
            // Request Schemas
            RegisterSchema,
            LoginSchema,

            // Response Schemas
            AuthResponse,
            FilteredUser,
            Role,
            TokenResponse,
            TokenDetails,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    ),
    info(
        title = "Starter Kit REST API Axum",
        version = "0.1.0",
        description = "A starter kit for building RESTful APIs with Rust, Axum, SQLx, and PostgreSQL.",
        contact(
            name = "Your Name",
            url = "http://your-website.com",
            email = "you@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
pub struct ApiDoc;