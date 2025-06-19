mod routes;
mod models;
mod middleware;


use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;

use crate::{routes::{auth, protected}, middleware::auth::auth_middleware};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "Auth API", description = "A simple auth API"),
        paths(
            auth::login,
            protected::admin_route
        ),
        components(schemas(
            crate::models::User,
            crate::models::Role,
            crate::models::LoginRequest,
            crate::models::LoginResponse
        ))
    )]
    struct ApiDoc;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(auth::login))
        .route("/admin", get(protected::admin_route))
        .layer(axum::middleware::from_fn(auth_middleware))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:30001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
