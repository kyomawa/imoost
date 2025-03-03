use actix_web::web;
use crate::handlers::{ home, healthcheck, image };

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home).service(healthcheck).service(image);
}
