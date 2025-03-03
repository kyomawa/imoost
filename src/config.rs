use dotenv::dotenv;
use std::env;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Config {
    pub imgproxy_url: String,
    pub allowed_domains: Vec<String>,
    pub imgproxy_key: Option<String>,
    pub imgproxy_salt: Option<String>,
    pub imgproxy_source_url_encryption_key: Option<String>,
    pub imgproxy_iv_key: Option<String>,
}

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
        let imgproxy_source_url_encryption_key = env
            ::var("IMGPROXY_SOURCE_URL_ENCRYPTION_KEY")
            .ok();
        let imgproxy_iv_key = env::var("IMGPROXY_IV_KEY").ok();

        Config {
            imgproxy_url,
            allowed_domains,
            imgproxy_key,
            imgproxy_salt,
            imgproxy_source_url_encryption_key,
            imgproxy_iv_key,
        }
    }
}
