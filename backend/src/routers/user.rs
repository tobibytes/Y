use axum::{extract::Multipart, http::HeaderMap, response::{IntoResponse, Response} };

use crate::controllers::{ user::USERCONTROLLER, AUTHCONTROLLER};



pub async fn onboard_route(headers: HeaderMap, mut data: Multipart) -> impl IntoResponse {
    let id_res = AUTHCONTROLLER.extract_cookies(headers).await;
    match id_res {
        Ok(id) => {
            return USERCONTROLLER.onboard(id, data).await
        }
        Err(e) => {
            return e.into_response()     }
    }
}