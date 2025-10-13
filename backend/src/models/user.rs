use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct UserRow {
    pub id: i32,
    pub email: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub profile_picture: Option<String>,
    pub is_verified: bool,
    pub is_onboarded: bool,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct IdRow {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadedFile {
    pub filename: Option<String>,
    pub content_type: Option<String>,
    #[serde(with = "serde_bytes")] // encodes as base64 if you ever serialize to JSON
    pub bytes: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OnboardData {
    pub bio: Option<String>,
    pub username: Option<String>,
    pub profile_picture: Option<UploadedFile>,
    pub dob: Option<NaiveDate>
}