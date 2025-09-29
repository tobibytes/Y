use redis::Commands;
use tracing::error;

use crate::secrets::SECRET_MANAGER;


pub struct RedisClient {
    connection : redis::Connection
}

impl RedisClient {
     fn new() -> Self {
        let redis_url = SECRET_MANAGER.get("REDIS_URL");
        let client_con = redis::Client::open(redis_url).expect("Failed to create Redis client");
        let connection= client_con.get_connection().expect("Failed to connect to Redis");
        RedisClient { connection }
    }

   pub fn get(self: &mut Self, key: &str) -> String {
    let result = self.connection.get(key);
    match result {
        Ok(value) => value,
        Err(err) => {
            error!("Error getting key {}: {}", key, err);
            None.unwrap()
        }
    }
   }
   pub fn set(self: &mut Self, key: &str, value: &str) {
    let result: redis::RedisResult<()> = self.connection.set(key, value);
    if let Err(err) = result {
        error!("Error setting key {}: {}", key, err);
    }
   }
}

pub static REDISCLIENT: once_cell::sync::Lazy<std::sync::Mutex<RedisClient>> = once_cell::sync::Lazy::new(|| {
    std::sync::Mutex::new(RedisClient::new())
});