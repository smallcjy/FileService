use actix_web::HttpResponse;


pub async fn oss_temp_credential() -> HttpResponse {
    HttpResponse::Ok().json("oss temp credential")
}

pub async fn upload() -> HttpResponse {
    HttpResponse::Ok().json("upload")
}

pub async fn download() -> HttpResponse {
    HttpResponse::Ok().json("download")
}