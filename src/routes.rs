use actix_web::web;

use crate::handlers::{ health, home, image };

// =========================================================================================

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home::home).service(health::healthcheck).service(image::image).service(image::test);
}

// =========================================================================================
