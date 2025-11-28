use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[sqlx(try_from = "String")]
    pub role: Role,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema, PartialEq, PartialOrd, Eq, Ord)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
}

impl TryFrom<String> for Role {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "user" => Ok(Role::User),
            "admin" => Ok(Role::Admin),
            _ => Err("Invalid role"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilteredUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub is_email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for FilteredUser {
    fn from(user: User) -> Self {
        FilteredUser {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            is_email_verified: user.is_email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}