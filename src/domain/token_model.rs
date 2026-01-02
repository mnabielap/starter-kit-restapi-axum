use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenDetails {
    pub token: String,
    pub expires: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    #[serde(rename = "access")]
    pub access_token: TokenDetails,
    #[serde(rename = "refresh")]
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
#[sqlx(type_name = "token_type", rename_all = "camelCase")]
pub enum TokenType {
    Refresh,
    ResetPassword,
    VerifyEmail,
}