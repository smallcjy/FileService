use actix_web::web;
use crate::controller::file::{download, oss_temp_credential, upload};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("api/file/v1")
                .route("/oss-temp-credential", web::get().to(oss_temp_credential))
                .route("/upload", web::post().to(upload))
                .route("/download", web::get().to(download))
    );
}