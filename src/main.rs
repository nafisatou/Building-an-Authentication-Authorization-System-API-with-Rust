use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod middleware;
pub mod models;
pub mod routes;

use crate::{
    middleware::auth::auth_middleware,
    routes::{auth, protected}, // ✅ Fix: import `register`
};

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<models::User>>>,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "Auth API", description = "A simple auth API"),
        paths(
            auth::login,     // ✅ Fix: correct path reference
            protected::admin_route,
            auth::register
        ),
        components(schemas(
            models::User,
            models::Role,
            models::LoginRequest,
            models::LoginResponse
        ))
    )]
    struct ApiDoc;

    let state = AppState {
        users: Arc::new(Mutex::new(vec![])),
    };

    let app = Router::new()
        .route("/admin", get(protected::admin_route))
        .layer(axum::middleware::from_fn(auth_middleware))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/register", post(routes::auth::register)) // ✅ Fix: correct route function
        .route("/login", post(auth::login))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
