use crate::{
    db::fetch_one_with,
    models::{general::ErrorResponse, user::OnboardData},
    secrets::SECRET_MANAGER,
};
use axum::{
    Json,
    extract::{Multipart, multipart::Field},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, NaiveDate};
use once_cell::sync::Lazy;
use reqwest::{
    Client,
    multipart::{Form, Part},
};
use std::path::Path;
use tracing::error;

static MEDIA_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub struct UserController;

impl UserController {
    pub async fn onboard(self: &Self, id: i32, mut data: Multipart) -> Response {
        let mut bio: Option<String> = None;
        let mut username: Option<String> = None;
        let mut profile_picture: Option<String> = None;
        let mut dob: Option<NaiveDate> = None;

        while let Ok(Some(field)) = data.next_field().await {
            let name = field.name().map(str::to_string);
            match name.as_deref() {
                Some("bio") => {
                    if let Ok(bytes) = field.bytes().await {
                        bio = String::from_utf8(bytes.to_vec()).ok()
                    }
                }
                Some("username") => {
                    if let Ok(bytes) = field.bytes().await {
                        username = String::from_utf8(bytes.to_vec()).ok()
                    }
                }
                Some("dob") => {
                    if let Ok(bytes) = field.bytes().await {
                        if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                            dob = self.parse_birthdate(&s)
                        }
                    }
                }
                Some("profile") if profile_picture.is_none() => {
                    match self.upload_profile_picture(id, field).await {
                        Ok(url) => profile_picture = Some(url),
                        Err(err) => {
                            error!("Failed to upload profile picture for user {}: {}", id, err);
                            return ErrorResponse::new(
                                "Failed to upload profile picture".to_string(),
                                StatusCode::BAD_GATEWAY,
                            )
                            .into_response();
                        }
                    }
                }
                _ => {
                    // Ignore unexpected fields for now.
                }
            }
        }
        const UPSERT_USER_QUERY: &str = r#"
            UPDATE "user" 
            SET bio = $1, username = $2, dob = $3, profile_picture = $4, is_onboarded = $5, onboarding_complete_at = NOW()
            WHERE id = $6
            RETURNING bio, username, dob, profile_picture;
        "#;
        let result_q = fetch_one_with::<OnboardData>(UPSERT_USER_QUERY, |q| {
            q.bind(bio.clone())
                .bind(username)
                .bind(dob)
                .bind(profile_picture)
                .bind(true)
                .bind(id)
        })
        .await;
        match result_q {
            Err(err) => {
                error!("Failed to upsert user {}: {}", id, err);
                return ErrorResponse::new(
                    "Failed to update user data".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
            Ok(data) => return  (StatusCode::OK, Json(data)).into_response(),
        }
    }

    async fn upload_profile_picture(
        self: &Self,
        user_id: i32,
        field: Field<'_>,
    ) -> Result<String, String> {
        let base_url = SECRET_MANAGER.get("MEDIA_SERVICE_URL");
        if base_url.trim().is_empty() {
            return Err("Media service URL is not configured".to_string());
        }

        let filename = field.file_name().map(str::to_string);
        let content_type = field.content_type().map(str::to_string);
        let extension = Self::infer_extension(filename.as_ref(), content_type.as_ref());
        let fallback_filename = format!("user_{}_profile.{}", user_id, extension.as_str());
        let chosen_filename = filename
            .clone()
            .unwrap_or_else(|| fallback_filename.clone());
        let final_content_type = Self::choose_content_type(content_type.as_ref(), &extension);

        let bytes = field
            .bytes()
            .await
            .map_err(|err| format!("Failed to read profile upload: {}", err))?;
        if bytes.is_empty() {
            return Err("Uploaded file was empty".to_string());
        }

        let bytes_vec: Vec<u8> = bytes.into_iter().collect();
        let mut part = Part::bytes(bytes_vec).file_name(chosen_filename);
        part = part
            .mime_str(&final_content_type)
            .map_err(|err| format!("Failed to set content type: {}", err))?;
        let form = Form::new()
            .text("extension", extension.clone())
            .text("content_type", final_content_type.clone())
            .part("file", part);

        let endpoint = format!("{}/file", base_url.trim_end_matches('/'));
        let client = &*MEDIA_CLIENT;
        let response = client
            .post(endpoint)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Failed to call media service: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Media service returned {}: {}", status, body));
        }

        response
            .json::<String>()
            .await
            .map_err(|e| format!("Failed to parse media service response: {}", e))
    }

    fn infer_extension(filename: Option<&String>, content_type: Option<&String>) -> String {
        if let Some(name) = filename {
            if let Some(ext) = Path::new(name).extension().and_then(|ext| ext.to_str()) {
                let trimmed = ext.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_ascii_lowercase();
                }
            }
        }

        match content_type.map(|s| s.as_str()) {
            Some("image/jpeg") | Some("image/jpg") => "jpg".to_string(),
            Some("image/png") => "png".to_string(),
            Some("image/gif") => "gif".to_string(),
            Some("image/webp") => "webp".to_string(),
            Some("application/pdf") => "pdf".to_string(),
            _ => "bin".to_string(),
        }
    }

    fn choose_content_type(content_type: Option<&String>, extension: &str) -> String {
        let sanitized = content_type
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && s.contains('/'))
            .map(str::to_string);

        sanitized.unwrap_or_else(|| Self::extension_to_mime(extension).to_string())
    }

    fn extension_to_mime(extension: &str) -> &'static str {
        match extension {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" | "svgz" => "image/svg+xml",
            "webp" => "image/webp",
            "pdf" => "application/pdf",
            "txt" => "text/plain",
            "json" => "application/json",
            "xml" => "application/xml",
            "mp4" => "video/mp4",
            "mp3" => "audio/mpeg",
            _ => "application/octet-stream",
        }
    }

    fn parse_birthdate(self: &Self, s: &str) -> Option<NaiveDate> {
        // Prefer plain date (no TZ issues)
        if let Ok(d) = NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d") {
            return Some(d);
        }
        // Fallback: take the date part of a timestamp
        if let Ok(dt) = DateTime::parse_from_rfc3339(s.trim()) {
            return Some(dt.date_naive());
        }
        None
    }
}

pub static USERCONTROLLER: Lazy<UserController> = Lazy::new(|| UserController);
