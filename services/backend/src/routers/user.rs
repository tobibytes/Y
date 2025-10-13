use axum::{
    extract::Multipart,
    http::HeaderMap,
    response::{IntoResponse },
};

use crate::controllers::{AUTHCONTROLLER, user::USERCONTROLLER};

pub async fn onboard_route(headers: HeaderMap,  data: Multipart) -> impl IntoResponse {
    let id_res = AUTHCONTROLLER.extract_cookies(headers).await;
    match id_res {
        Ok(id) => return USERCONTROLLER.onboard(id, data).await,
        Err(e) => return e.into_response(),
    }
}
