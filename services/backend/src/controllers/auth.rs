use axum::Json;
use axum::http::StatusCode;
use axum::http::header::COOKIE;
use axum::http::{HeaderMap, HeaderValue, header::SET_COOKIE};
use axum::response::{IntoResponse, Response};
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header as JwtHeader, Validation, decode, encode,
};
use oauth_axum::{CustomProvider, OAuthClient, providers::google::GoogleProvider};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::db::fetch_one_with;
use crate::models::{
    auth::{AuthCallbackResponse, GoogleTokenResponse, GoogleUserInfo, OauthUrlResponse},
    general::ErrorResponse,
    user::{IdRow, UserRow},
};
use crate::redis::REDISCLIENT;
use crate::secrets::SECRET_MANAGER;

pub struct AuthController {
    google_provider: CustomProvider,
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
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

    pub async fn get_google_oauth_url(&self) -> Response {
        let mut url = String::from("");
        self.google_provider
            .clone()
            .generate_url(
                Vec::from([
                    "openid".to_string(),
                    "email".to_string(),
                    "profile".to_string(),
                ]),
                |state_e| async {
                    url = match state_e.url_generated {
                        Some(u) => {
                            match REDISCLIENT.lock() {
                                Ok(mut client) => {
                                    client.set(&state_e.state, &state_e.verifier);
                                }
                                Err(poisoned) => {
                                    error!(
                                        "Redis mutex poisoned while saving verifier: {:?}",
                                        poisoned
                                    );
                                }
                            }
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
        Json(OauthUrlResponse { url }).into_response()
    }

    pub async fn google_oauth_callback(
        &self,
        queries: std::collections::HashMap<String, String>,
    ) -> Response {
        info!("Google OAuth callback queries: {:?}", queries);
        let code = match queries.get("code") {
            Some(v) => v.to_string(),
            None => {
                error!("Missing 'code' in callback query");
                return ErrorResponse::new(
                    "Missing 'code' in callback query".to_string(),
                    StatusCode::BAD_REQUEST,
                )
                .into_response();
            }
        };
        let state = match queries.get("state") {
            Some(v) => v.to_string(),
            None => {
                error!("Missing 'state' in callback query");
                return ErrorResponse::new(
                    "Missing 'state' in callback query".to_string(),
                    StatusCode::BAD_REQUEST,
                )
                .into_response();
            }
        };

        let verifier = match REDISCLIENT.lock() {
            Ok(mut client) => client.get(&state),
            Err(poisoned) => {
                error!(
                    "Redis mutex poisoned while reading verifier: {:?}",
                    poisoned
                );
                return ErrorResponse::new(
                    "Server concurrency error".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        };

        // Exchange the code for tokens manually against Google's token endpoint
        let client_id = SECRET_MANAGER.get("GOOGLE_CLIENT_ID");
        let client_secret = SECRET_MANAGER.get("GOOGLE_CLIENT_SECRET");
        let redirect_uri = SECRET_MANAGER.get("GOOGLE_REDIRECT_URL");

        if verifier.is_empty() {
            error!("Missing PKCE code_verifier for state={}", state);
            return ErrorResponse::new(
                format!("Missing PKCE code_verifier for state={}", state),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
        }

        let http = reqwest::Client::new();
        let token_resp = http
            .post("https://oauth2.googleapis.com/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&[
                ("code", code.as_str()),
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("redirect_uri", redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
                ("code_verifier", verifier.as_str()),
            ])
            .send()
            .await;

        let token_resp = match token_resp {
            Ok(r) => r,
            Err(_) => {
                error!("Token exchange request failed (network)");
                return ErrorResponse::new(
                    "Token exchange request failed (network)".to_string(),
                    StatusCode::BAD_REQUEST,
                )
                .into_response();
            }
        };

        let status = token_resp.status();
        let token_json = token_resp.text().await.unwrap_or_else(|_| "{}".to_string());
        if !status.is_success() {
            error!(
                "Token exchange failed: status={} body={}",
                status.as_u16(),
                token_json
            );
            return ErrorResponse::new(
                format!(
                    "Token exchange failed: status={}, body={}",
                    status.as_u16(),
                    token_json
                ),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
        }

        let token_parsed: Result<GoogleTokenResponse, _> = serde_json::from_str(&token_json);
        let (access_token_str, expires_in_opt, refresh_token_opt) = match token_parsed {
            Ok(t) => (t.access_token, t.expires_in, t.refresh_token),
            Err(e) => {
                info!(
                    "Token body not JSON object, falling back to raw token string; parse_err={:?}; body={}",
                    e, token_json
                );
                (token_json.trim_matches('"').to_string(), None, None)
            }
        };

        // Fetch Google user info
        let client = reqwest::Client::new();
        let mut userinfo: GoogleUserInfo = match client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(&access_token_str)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<GoogleUserInfo>().await {
                Ok(u) => u,
                Err(e) => {
                    error!("Failed to parse userinfo (v2): {:?}", e);
                    GoogleUserInfo {
                        id: None,
                        sub: None,
                        email: None,
                        verified_email: None,
                        name: None,
                        given_name: None,
                        family_name: None,
                        picture: None,
                    }
                }
            },
            Err(e) => {
                error!("Failed to fetch userinfo (v2): {:?}", e);
                GoogleUserInfo {
                    id: None,
                    sub: None,
                    email: None,
                    verified_email: None,
                    name: None,
                    given_name: None,
                    family_name: None,
                    picture: None,
                }
            }
        };

        // Fallback to OpenID userinfo if needed
        if userinfo.id.is_none() && userinfo.sub.is_none() {
            if let Ok(resp) = client
                .get("https://openidconnect.googleapis.com/v1/userinfo")
                .bearer_auth(&access_token_str)
                .send()
                .await
            {
                if let Ok(u) = resp.json().await {
                    userinfo = u;
                }
            }
        }

        let google_user_id = userinfo
            .id
            .clone()
            .or(userinfo.sub.clone())
            .unwrap_or_default();
        info!("{:?}", userinfo);

        let email = userinfo.email.clone();
        let name = userinfo.name.clone();
        let avatar = userinfo.picture.clone();

        let token_expires_at: Option<NaiveDateTime> =
            expires_in_opt.map(|sec| (Utc::now() + Duration::seconds(sec)).naive_utc());

        const UPSERT_USER_QUERY: &str = r#"
            INSERT INTO "user" (
                "email",
                "provider",
                "provider_user_id",
                "access_token",
                "refresh_token",
                "token_expires_at",
                "name",
                "profile_picture",
                "created_at",
                "updated_at"
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8, NOW(), NOW())
            ON CONFLICT ("provider","provider_user_id") DO UPDATE
            SET
                "email" = EXCLUDED."email",
                "access_token" = EXCLUDED."access_token",
                "refresh_token" = EXCLUDED."refresh_token",
                "token_expires_at" = EXCLUDED."token_expires_at",
                "name" = EXCLUDED."name",
                "profile_picture" = EXCLUDED."profile_picture",
                "updated_at" = NOW()
            RETURNING id
        "#;
        let result_q = fetch_one_with::<IdRow>(UPSERT_USER_QUERY, |q| {
            q.bind(email.clone())
                .bind("google")
                .bind(google_user_id)
                .bind(access_token_str.clone())
                .bind(refresh_token_opt.clone())
                .bind(token_expires_at)
                .bind(name.clone())
                .bind(avatar.clone())
        })
        .await;
        match result_q {
            Ok(row) => {
                let user_id = row.id as i64;

                // Sign JWT for session
                let exp = (Utc::now() + Duration::days(7)).timestamp() as usize;
                let claims = Claims { sub: user_id, exp };
                let jwt = match encode(
                    &JwtHeader::default(),
                    &claims,
                    &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
                ) {
                    Ok(t) => t,
                    Err(e) => {
                        error!("Failed to sign JWT: {:?}", e);
                        return ErrorResponse::new(
                            "Failed to sign session".to_string(),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                        .into_response();
                    }
                };

                // Set secure HttpOnly cookie (Secure only for https FRONTEND_URL)
                let frontend_url = SECRET_MANAGER.get("FRONTEND_URL");
                let secure_flag = frontend_url.starts_with("https://");
                let cookie = format!(
                    "session={}; Path=/; HttpOnly; SameSite=Lax{}; Max-Age={}",
                    jwt,
                    if secure_flag { "; Secure" } else { "" },
                    60 * 60 * 24 * 7
                );

                let mut headers = HeaderMap::new();
                let set_cookie = match HeaderValue::from_str(&cookie) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("Invalid Set-Cookie header value: {:?}", e);
                        return ErrorResponse::new(
                            "Server cookie error".to_string(),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                        .into_response();
                    }
                };
                headers.insert(SET_COOKIE, set_cookie);

                (
                    headers,
                    Json(AuthCallbackResponse {
                        ok: true,
                        user_id,
                        email,
                        name,
                        avatar,
                    }),
                )
                    .into_response()
            }
            Err(_) => {
                error!("Could not get Auth Callback Response");
                return ErrorResponse::new(
                    "Could not get Auth Callback Response".to_string(),
                    StatusCode::BAD_REQUEST,
                )
                .into_response();
            }
        }
    }

    pub async fn extract_cookies(
        self: &Self,
        headers: HeaderMap,
    ) -> Result<i32, (StatusCode, Json<ErrorResponse>)> {
        let cookies = match headers.get(COOKIE).and_then(|v| v.to_str().ok()) {
            Some(v) => v,
            None => {
                return Err(ErrorResponse::new(
                    "Missing cookies in header".to_string(),
                    StatusCode::UNAUTHORIZED,
                ));
            }
        };
        let session_jwt = cookies
            .split(';')
            .map(|p| p.trim())
            .find_map(|p| p.strip_prefix("session="))
            .map(|v| v.to_string());
        let session_jwt = match session_jwt {
            Some(v) if !v.is_empty() => v,
            _ => {
                return Err(ErrorResponse::new(
                    "Missing session cookie".to_string(),
                    StatusCode::UNAUTHORIZED,
                ));
            }
        };
        let jwt_secret = SECRET_MANAGER.get("JWT_SECRET");
        let token_data = match decode::<Claims>(
            &session_jwt,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(d) => d,
            Err(e) => {
                error!("Invalid session token: {:?}", e);
                return Err(ErrorResponse::new(
                    "Invalid session".into(),
                    StatusCode::UNAUTHORIZED,
                ));
            }
        };
        let user_id_i32 = token_data.claims.sub as i32;
        return Ok(user_id_i32);
    }

    pub async fn me(self: &Self, id: i32) -> Result<UserRow, (StatusCode, Json<ErrorResponse>)> {
        let query_str = r#"
        SELECT id, email, name, username, profile_picture, is_verified, is_onboarded
        FROM "user" 
        WHERE id = $1
        "#;
        let result_q = fetch_one_with::<UserRow>(query_str, |q| q.bind(id)).await;
        match result_q {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("Could not get user info: {:?}", e);
                return Err(ErrorResponse::new(
                    format!("Could not get user info: {:?}", e),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }
    }
}

pub static AUTHCONTROLLER: Lazy<AuthController> = Lazy::new(|| AuthController::new());
