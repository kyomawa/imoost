mod config;
mod handlers;
mod routes;

use actix_web::{ HttpServer, App, web };
use config::Config;

// =========================================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    println!("▶️  Starting server at http://0.0.0.0:8000");
    println!("🔗 Using imgproxy URL: {}", config.imgproxy_url);

    HttpServer::new(move || {
        App::new().configure(routes::init_routes).app_data(web::Data::new(config.clone()))
    })
        .bind(("0.0.0.0", 8000))?
        .run().await
}

// =========================================================================================
