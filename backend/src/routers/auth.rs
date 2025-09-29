use axum::extract::Query;
use axum::response::IntoResponse;

use crate::controllers::AUTHCONTROLLER;

pub async fn get_google_oauth_url_route() -> impl IntoResponse {
    AUTHCONTROLLER.get_google_oauth_url().await
}

pub async fn google_oauth_callback_route(Query(queries): Query<std::collections::HashMap<String, String>>) -> impl IntoResponse {
    AUTHCONTROLLER.google_oauth_callback(queries).await
}
