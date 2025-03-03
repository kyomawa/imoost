use actix_web::{ get, HttpResponse, Responder };

// =========================================================================================

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("✅ Server is up and running!")
}

// =========================================================================================
