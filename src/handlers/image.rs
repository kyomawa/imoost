use actix_web::{ get, web, HttpResponse, Responder };
use crate::config::Config;
use url::Url;
use std::collections::HashMap;
use awc::Client;
use openssl::symm::{ encrypt, Cipher };
use openssl::sign::Signer;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use base64::{ engine::general_purpose, Engine as _ };
use hex;

// =========================================================================================

#[get("/image/{url:.*}")]
pub async fn image(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    config: web::Data<Config>
) -> impl Responder {
    let src = path.into_inner();
    let parsed_url = match validate_image_url(&src) {
        Ok(u) => u,
        Err(resp) => {
            return resp;
        }
    };

    let origin = parsed_url.host_str().unwrap_or("");
    if !is_domain_allowed(origin, &config.allowed_domains) {
        return HttpResponse::Forbidden().body(format!("⚠️ Domain ({}) not allowed.", origin));
    }

    let (width, height, quality) = get_transformation_params(&query);

    let preset = "pr:sharp";
    let processing_path = format!("/resize:fill:{}:{}/q:{}/plain/{}", width, height, quality, src);
    let final_url = compute_final_url(&src, &width, &quality, &processing_path, &config);

    let client = Client::default();
    let response = client
        .get(&final_url)
        .insert_header(("Accept", "image/avif,image/webp,image/apng,*/*"))
        .send().await;

    match response {
        Ok(mut resp) if resp.status().is_success() => {
            let headers = resp.headers().clone();
            let body_bytes = resp.body().await.unwrap_or_else(|_| web::Bytes::new());
            let mut actix_resp = HttpResponse::Ok();
            for (key, value) in headers.iter() {
                if let Ok(val_str) = value.to_str() {
                    actix_resp.insert_header((key.as_str(), val_str));
                }
            }
            actix_resp.insert_header(("Server", "Imoost"));
            actix_resp.body(body_bytes)
        }
        Ok(resp) =>
            HttpResponse::build(resp.status()).body("❌ Error fetching image from imgproxy"),
        Err(e) => {
            println!("❌ Error fetching image: {:?}", e);
            HttpResponse::InternalServerError().body("❌ Error resizing image.")
        }
    }
}

// =========================================================================================

fn validate_image_url(src: &str) -> Result<Url, HttpResponse> {
    Url::parse(src).map_err(|_| HttpResponse::BadRequest().body("⚠️ Invalid image URL"))
}

// =========================================================================================

fn is_domain_allowed(origin: &str, allowed_domains: &Vec<String>) -> bool {
    allowed_domains.iter().any(|domain| {
        if domain == "*" {
            true
        } else if domain == origin {
            true
        } else if domain.starts_with("*.") && origin.ends_with(&domain[2..]) {
            true
        } else {
            false
        }
    })
}

// =========================================================================================

fn get_transformation_params(query: &HashMap<String, String>) -> (String, String, String) {
    let default_width = "0".to_string();
    let default_height = "0".to_string();
    let default_quality = "75".to_string();

    let width = query.get("width").unwrap_or(&default_width).to_string();
    let height = query.get("height").unwrap_or(&default_height).to_string();
    let quality = query.get("quality").unwrap_or(&default_quality).to_string();

    (width, height, quality)
}

// =========================================================================================

fn compute_final_url(
    src: &str,
    width: &str,
    quality: &str,
    processing_path: &str,
    config: &Config
) -> String {
    let base_url = &config.imgproxy_url;
    if
        let (Some(enc_key), Some(iv_key)) = (
            &config.imgproxy_source_url_encryption_key,
            &config.imgproxy_iv_key,
        )
    {
        if let Ok(encrypted) = encrypt_source_url(src, enc_key, iv_key) {
            return format!(
                "{}/unsafe/enc/{}?width={}&quality={}",
                base_url,
                encrypted,
                width,
                quality
            );
        }
    }
    if let (Some(sign_key), Some(sign_salt)) = (&config.imgproxy_key, &config.imgproxy_salt) {
        if let Ok(signature) = sign_url(processing_path, sign_key, sign_salt) {
            return format!("{}/{}/{}", base_url, signature, processing_path);
        }
    }
    format!("{}{}", base_url, processing_path)
}

// =========================================================================================

fn encrypt_source_url(
    src: &str,
    key_hex: &str,
    iv_key: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let key = hex::decode(key_hex)?;

    let pkey = PKey::hmac(iv_key.as_bytes())?;
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
    signer.update(src.as_bytes())?;
    let hmac = signer.sign_to_vec()?;
    let iv = &hmac[..16];

    let cipher = match key.len() {
        16 => Cipher::aes_128_cbc(),
        24 => Cipher::aes_192_cbc(),
        32 => Cipher::aes_256_cbc(),
        _ => {
            return Err("Invalid key length".into());
        }
    };

    let ciphertext = encrypt(cipher, &key, Some(iv), src.as_bytes())?;
    let mut combined = iv.to_vec();
    combined.extend(ciphertext);
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(&combined))
}

// =========================================================================================

fn sign_url(
    path: &str,
    key_hex: &str,
    salt_hex: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let key = hex::decode(key_hex)?;
    let salt = hex::decode(salt_hex)?;
    let mut data = salt;
    data.extend_from_slice(path.as_bytes());

    let pkey = PKey::hmac(&key)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
    signer.update(&data)?;
    let signature = signer.sign_to_vec()?;
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(&signature))
}

// =========================================================================================

#[get("/test")]
pub async fn test() -> impl Responder {
    let src =
        "https://cloud.api.bryancellier.fr/4cite-akkorhotel/hotels/67c497f6fe56161ca9eae8c3/67c497f6291a940e35d4f6b4";
    let key = "314151d85d73af460cc5395188f75c2b6bc52257f52be97be04e7f5694a6a977";
    let iv_key = "cdaddffe20279d72219fb7c2ae62d908";
    match encrypt_source_url(src, key, iv_key) {
        Ok(encrypted) => HttpResponse::Ok().body(format!("Encrypted: {}", encrypted)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// =========================================================================================
