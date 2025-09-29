use axum::{
    routing::{ get },
    Router,
};
use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};
mod routers;
mod controllers;
mod models;
mod db;
mod secrets;
mod redis;
use crate::secrets::{SECRET_MANAGER};
use routers::{ get_google_oauth_url_route, google_oauth_callback_route };
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer};
use axum::http::{header::{ ACCEPT,ACCESS_CONTROL_ALLOW_CREDENTIALS, AUTHORIZATION, CONTENT_TYPE, COOKIE}, HeaderValue, Method};
#[tokio::main]
async fn main() {
    let port = SECRET_MANAGER.get("PORT");
    let frontend_url = SECRET_MANAGER.get("FRONTEND_URL");
    fmt().with_env_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
    .with_target(false)
    .init(); // Vite uses VITE_ prefix

    let allowed_origin: HeaderValue = frontend_url.parse().expect("Invalid FRONTEND_URL for CORS");
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT, COOKIE, ACCESS_CONTROL_ALLOW_CREDENTIALS]);
    let app = Router::new()
        .route("/", get(root))
        .route("/auth/google/url", get(get_google_oauth_url_route))
        .route("/auth/google/callback", get(google_oauth_callback_route))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "WELCOME TO Y!"
}
