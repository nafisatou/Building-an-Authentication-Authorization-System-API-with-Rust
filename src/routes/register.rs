use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::hash_with_salt;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use utoipa::OpenApi;

use crate::middleware::auth::Claims;
use crate::models::{LoginRequest, LoginResponse, Role};
use crate::AppState;

const JWT_SALT: &[u8; 16] = b"your-secret-salt";

#[derive(OpenApi)]
#[openapi(paths(register), components(schemas(LoginRequest, LoginResponse)))]
pub struct AuthApi;


//The login function here


#[utoipa::path(
    post,
    path = "/register",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User regisered successfully", body = LoginResponse),
        (status = 401, description = "Bad request")
    )
)]
pub async fn register(
    State(mut state): State<AppState>,
    Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // In production, verify against a database
    if payload.username.is_empty() || payload.password.is_empty()  {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Username and password are required"})),
        )
            .into_response();
    }

    // Here you would typically hash the password and save the user to a database
       let hashed_password = hash_with_salt(payload.password.as_bytes(), bcrypt::DEFAULT_COST, *JWT_SALT).unwrap();

       let new_user = crate::models::User {
        id: state.users.len() as i32 + 1,
        username: payload.username,
        password: hashed_password.to_string(),
        role: Role::User,
       };

       state.users.push(new_user);

       ( 
        StatusCode::CREATED,
        Json(json!({"message": "User registered successfully"})),
      )
        .into_response()
}