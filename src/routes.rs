use actix_web::web;
use crate::controller::helloworld::say;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("api/hello-world")
                .route("/say", web::get().to(say))
    );
}