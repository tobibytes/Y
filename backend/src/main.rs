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

#[tokio::main]
async fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
    .with_target(false)
    .init();

    let app = Router::new()
        .route("/", get(root));

    let port = SECRET_MANAGER.get("PORT");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "WELCOME TO Y!"
}
