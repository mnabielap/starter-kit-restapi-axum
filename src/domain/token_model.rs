use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenDetails {
    pub token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub access_token: TokenDetails,
    pub refresh_token: TokenDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
    pub token_type: String,
}

#[derive(Debug, sqlx::Type, Clone, PartialEq)]

pub enum TokenType {
    Refresh,
    ResetPassword,
    VerifyEmail,
}