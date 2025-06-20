use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // In production, this should be hashed
    pub role: Role,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

// ✅ Added for registration feature
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Username of the new user
    pub username: String,
    /// Password for the new user
    pub password: String,
    /// Confirm password to validate match
    pub confirm_password: String,
}
