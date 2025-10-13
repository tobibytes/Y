use axum::extract::Query;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use tracing::debug;

use crate::controllers::AUTHCONTROLLER;

pub async fn get_google_oauth_url_route() -> impl IntoResponse {
    AUTHCONTROLLER.get_google_oauth_url().await
}

pub async fn google_oauth_callback_route(Query(queries): Query<std::collections::HashMap<String, String>>) -> impl IntoResponse {
    AUTHCONTROLLER.google_oauth_callback(queries).await
}

pub async fn me_route(headers: HeaderMap) -> impl IntoResponse {
    let id_res = AUTHCONTROLLER.extract_cookies(headers).await;
    match id_res {
        Ok(id) => {
            match AUTHCONTROLLER.me(id).await {
                Ok(res) => { 
                    debug!("{:?}", res);
                    return (StatusCode::OK, Json(res)).into_response()
                }
                Err(e) => { 
                    debug!("{:?}", e);
                    return e.into_response() 
                }
            }
        }
        Err(e) => {
            return e.into_response()
        }
    }
}