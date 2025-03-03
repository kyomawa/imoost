use dotenv::dotenv;
use std::env;

// =========================================================================================

#[derive(Clone)]
pub struct Config {
    pub imgproxy_url: String,
    pub allowed_domains: Vec<String>,
    pub imgproxy_key: Option<String>,
    pub imgproxy_salt: Option<String>,
}

// =========================================================================================

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let imgproxy_url = env
            ::var("IMGPROXY_URL")
            .unwrap_or_else(|_| "http://imgproxy:8080".to_string());

        let allowed_domains = env
            ::var("ALLOWED_DOMAINS")
            .unwrap_or_else(|_| "*".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let imgproxy_key = env::var("IMGPROXY_KEY").ok();
        let imgproxy_salt = env::var("IMGPROXY_SALT").ok();

        Config {
            imgproxy_url,
            allowed_domains,
            imgproxy_key,
            imgproxy_salt,
        }
    }
}

// =========================================================================================
