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