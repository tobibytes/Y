use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OauthUrlResponse {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: Option<i64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleUserInfo {
    pub id: Option<String>,     // googleapis.com/oauth2/v2
    pub sub: Option<String>,    // openidconnect.googleapis.com
    pub email: Option<String>,
    pub verified_email: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthCallbackResponse {
    pub ok: bool,
    pub user_id: i64,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
}
