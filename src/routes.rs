use actix_web::web;
use crate::controller::v1::file::service_config;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("/api/file")
                .configure(service_config
            )
    );
}