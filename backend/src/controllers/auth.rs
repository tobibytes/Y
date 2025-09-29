use axum::Json;
use axum::response::IntoResponse;
use once_cell::sync::Lazy;
use tracing::{ error, info};
use crate::redis::REDISCLIENT;
use crate::secrets::SECRET_MANAGER;
use oauth_axum::{providers::google::GoogleProvider, CustomProvider, OAuthClient };
use crate::models::{auth::OauthUrlResponse};
pub struct AuthController {
    google_provider: CustomProvider,
    jwt_secret: String,
}

impl AuthController {
    fn new() -> Self {

        AuthController {
            google_provider: GoogleProvider::new(
                SECRET_MANAGER.get("GOOGLE_CLIENT_ID"),
                SECRET_MANAGER.get("GOOGLE_CLIENT_SECRET"),
                SECRET_MANAGER.get("GOOGLE_REDIRECT_URL"),
            ),
            jwt_secret: SECRET_MANAGER.get("JWT_SECRET"),
        }
    }

pub async fn get_google_oauth_url(&self) -> impl IntoResponse {
    let mut url = String::from("");
    self.google_provider.clone().generate_url(Vec::from([
        "openid".to_string(), "email".to_string(), "profile".to_string()
        ]), 
        |state_e| async {
            url = match state_e.url_generated {
                Some(u) => {
                REDISCLIENT.lock().unwrap().set(&state_e.state, &state_e.verifier);
                u
            }
            None => {
                error!("Error generating Google OAuth URL: {:?}", state_e);
                "".to_string()
        }
    };
    },
)
    .await
    .ok(); 
    Json(OauthUrlResponse { url })
}
    pub async fn google_oauth_callback(&self, queries: std::collections::HashMap<String, String>) -> String {
        info!("Google OAuth callback queries: {:?}", queries);
        let code = queries.get("code").unwrap().to_string();
        let state = queries.get("state").unwrap();
        let verifier = REDISCLIENT.lock().unwrap().get(&state);
        self.google_provider.clone().generate_token(code, verifier)
        .await
        .ok()
        .unwrap()

    }
}

pub static AUTHCONTROLLER: Lazy<AuthController> = Lazy::new(|| {
    AuthController::new()
});