use actix_web::{ get, web, HttpResponse, Responder };
use crate::config::Config;
use url::Url;
use std::collections::HashMap;
use awc::Client;

#[get("/")]
pub async fn home() -> impl Responder {
    let body =
        r#"
    <!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="description" content="Imoost is an open-source, self-hosted image optimization service built for Next.js. It leverages imgproxy to dynamically transform, resize, and compress your images." />
    <meta name="author" content="Bryan Cellier, https://bryancellier.fr" />
    <title>Imoost</title>
    <script src="https://cdn.tailwindcss.com"></script>
  </head>
  <body>
    <main class="flex min-h-dvh items-center bg-[#FAFAFA] justify-center p-4">
      <div class="flex flex-col items-center gap-y-6">
        <div class="space-y-1">
          <h3 class="text-center text-4xl font-bold">Imoost</h3>
          <p class="text-center max-w-[64rem]">
            Imoost is an open-source, self-hosted image optimization service
            built for Next.js. It leverages imgproxy to dynamically transform,
            resize, and compress your images for faster load times and better
            performance—all while giving you full control over your image
            infrastructure.
          </p>
        </div>
        <a
          class="rounded bg-black px-4 py-2 text-white hover:bg-black/85 transition-colors duration-150"
          href="https://github.com/kyomawa/imoost"
          >Link to the repo</a
        >
      </div>
    </main>
  </body>
</html>
"#;
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("✅ Server is up and running!")
}

#[get("/image/{url:.*}")]
pub async fn image(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    config: web::Data<Config>
) -> impl Responder {
    let src = path.into_inner();

    let parsed_url = match Url::parse(&src) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::BadRequest().body("⚠️ Invalid image URL");
        }
    };
    let origin = parsed_url.host_str().unwrap_or("");

    let is_allowed = config.allowed_domains.iter().any(|domain| {
        if domain == "*" {
            true
        } else if domain == origin {
            true
        } else if domain.starts_with("*.") && origin.ends_with(&domain[2..]) {
            true
        } else {
            false
        }
    });
    if !is_allowed {
        return HttpResponse::Forbidden().body(
            format!("⚠️ Domain ({}) not allowed. More details here: https://github.com/kyomawa/imoost", origin)
        );
    }

    let default_width = "0".to_string();
    let default_height = "0".to_string();
    let default_quality = "75".to_string();

    let width = query.get("width").unwrap_or(&default_width);
    let height = query.get("height").unwrap_or(&default_height);
    let quality = query.get("quality").unwrap_or(&default_quality);

    let preset = "pr:sharp";
    let request_url = format!(
        "{}/{}/resize:fill:{}:{}/q:{}/plain/{}",
        config.imgproxy_url,
        preset,
        width,
        height,
        quality,
        src
    );

    let client = Client::default();
    let response = client
        .get(&request_url)
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
