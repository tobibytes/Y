use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use tracing::debug;
pub static SECRET_MANAGER: Lazy<SecretManager> = Lazy::new(|| SecretManager::new());

enum MODE {
    DEV,
    PROD,
}
#[derive(Debug)]
pub struct SecretManager {
    secrets: HashMap<String, String>,
}
impl SecretManager {
    fn new() -> Self {
        let mut secrets: HashMap<String, String> = HashMap::new();
        let mode = match env::var("MODE") {
            Ok(mode) if mode.to_lowercase() == "prod" => MODE::PROD,
            _ => MODE::DEV,
        };
        match mode {
            MODE::DEV => {
                secrets.insert("PORT".to_string(), "8001".to_string());
                secrets.insert(
                    "BACKEND_URL".to_string(),
                    "http://localhost:8001".to_string(),
                );
                secrets.insert("BACKEND_DOMAIN".to_string(), "localhost".to_string());
            }
            MODE::PROD => {
                secrets.insert("PORT".to_string(), env::var("PORT").unwrap_or_default());
                secrets.insert(
                    "FRONTEND_URL".to_string(),
                    env::var("FRONTEND_URL").unwrap_or_default(),
                );
                secrets.insert(
                    "BACKEND_URL".to_string(),
                    env::var("BACKEND_URL").unwrap_or_default(),
                );
            }
        }
        secrets.insert("JWT_SECRET".to_string(), "secret".to_string());
        secrets.insert(
            "STORAGE_ACCOUNT".to_string(),
            env::var("STORAGE_ACCOUNT").unwrap_or_default(),
        );
        secrets.insert(
            "STORAGE_ACCESS_KEY".to_string(),
            env::var("STORAGE_ACCESS_KEY").unwrap_or_default(),
        );
        secrets.insert(
            "STORAGE_CONTAINER".to_string(),
            env::var("STORAGE_CONTAINER").unwrap_or_default(),
        );
        debug!("Secrets loaded: {:?}", secrets);
        SecretManager { secrets }
    }

    pub fn get(&self, key: &str) -> String {
        self.secrets.get(key).cloned().unwrap_or_default()
    }
}
