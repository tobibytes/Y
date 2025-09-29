use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct OauthUrlResponse {
    pub url: String
}