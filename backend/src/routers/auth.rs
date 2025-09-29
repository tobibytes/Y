use axum::extract::Query;

use crate::controllers::AUTHCONTROLLER;


pub async fn get_google_oauth_url_route() -> String {
    AUTHCONTROLLER.get_google_oauth_url().await
}

pub async fn google_oauth_callback_route(Query(queries): Query<std::collections::HashMap<String, String>>) -> String{    
    AUTHCONTROLLER.google_oauth_callback(queries).await
}