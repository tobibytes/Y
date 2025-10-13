use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub ok: bool,
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: String, status_code: StatusCode) -> (StatusCode, Json<ErrorResponse>) {
        (status_code, Json(ErrorResponse { ok: false, error }))
    }
}
