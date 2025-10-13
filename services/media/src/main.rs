use crate::{
    secrets::SECRET_MANAGER,
    upload::{AZURESTORAGECONFIG, UploadMetadata},
};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use tracing::{Level, debug, error};
use tracing_subscriber::{EnvFilter, fmt};
mod secrets;
mod upload;
use tower_http::trace::TraceLayer;
#[tokio::main]
async fn main() {
    debug!("secrets: {:?}", SECRET_MANAGER);
    let port = SECRET_MANAGER.get("PORT");
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
        .with_target(false)
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/file", post(upload_route))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "WELCOME TO Y!"
}

async fn upload_route(mut multipart: Multipart) -> impl IntoResponse {
    let mut extension: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut upload_url: Option<String> = None;

    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                match field.name() {
                    Some("extension") => {
                        if let Ok(value) = field.text().await {
                            extension = Some(value);
                        }
                    }
                    Some("content_type") => {
                        if let Ok(value) = field.text().await {
                            content_type = Some(value);
                        }
                    }
                    Some("file") => {
                        let metadata = UploadMetadata {
                            extension: extension.clone(),
                            content_type: content_type.clone(),
                        };
                        match AZURESTORAGECONFIG.upload_stream(field, metadata).await {
                            Ok(url) => {
                                upload_url = Some(url);
                                break;
                            }
                            Err(err) => {
                                return (
                                    StatusCode::BAD_REQUEST,
                                    Json(format!("Error uploading: {}", err)),
                                )
                                    .into_response();
                            }
                        }
                    }
                    _ => {
                        // Ignore unknown fields
                    }
                }
            }
            Ok(None) => break,
            Err(err) => {
                error!("Failed to read multipart field: {}", err);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(format!("Error uploading: {}", err)),
                )
                    .into_response();
            }
        }
    }

    match upload_url {
        Some(url) => (StatusCode::OK, Json(url)).into_response(),
        None => (
            StatusCode::BAD_REQUEST,
            Json("Error uploading: missing file field".to_string()),
        )
            .into_response(),
    }
}
