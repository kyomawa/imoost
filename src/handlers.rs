use actix_web::{ get, web, HttpResponse, Responder };

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body(
        "Imoost is an open-source, self-hosted image optimization service built for Next.js. It leverages imgproxy to dynamically transform, resize, and compress your images for faster load times and better performance—all while giving you full control over your image infrastructure."
    )
}

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("✅ Server is up and running!")
}

#[get("/image/{url}")]
pub async fn image(path: web::Path<String>) -> impl Responder {
    let image_url = path.into_inner();
    HttpResponse::Ok().body(format!("Processing image: {}", image_url))
}
