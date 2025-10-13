use crate::{ models::user::{OnboardData, UploadedFile}};
use axum::{extract::Multipart, http::StatusCode, response::{IntoResponse, Response}, Json};
use chrono::{DateTime, NaiveDate};
use once_cell::sync::Lazy;

pub struct UserController;

impl UserController {
    pub async fn onboard(self: &Self, id: i32, mut data: Multipart) -> Response {
        let mut bio: Option<String> = None;
        let mut username: Option<String> = None;
        let mut profile: Option<UploadedFile> = None;
        let mut dob: Option<NaiveDate> = None;
    
        while let Ok(Some(field)) = data.next_field().await {
            let name = field.name().map(str::to_string);
            let filename = field.file_name().map(str::to_string);
            let content_type = field.content_type().map(str::to_string);
            let bytes = field.bytes().await.unwrap_or_default();
    
            match name.as_deref() {
                Some("bio") => bio = String::from_utf8(bytes.to_vec()).ok(),
                Some("username") => username = String::from_utf8(bytes.to_vec()).ok(),
                Some("dob") => {
                    if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                        dob = self.parse_birthdate(&s)
                    }
                },
                Some("profile") => {
                    profile = Some(UploadedFile {
                        filename,
                        content_type,
                        bytes: bytes.to_vec(), // keep in memory
                    });
                }
                _ => {
                    // ignore or collect extra fields as needed
                }
            }
        }
    
        (StatusCode::OK, Json(OnboardData { bio, username, dob, profile_picture: profile })).into_response()
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

pub static USERCONTROLLER:  Lazy<UserController> = Lazy::new(|| UserController);