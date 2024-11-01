use actix_web::HttpResponse;

pub async fn say() -> HttpResponse{
    return HttpResponse::Ok().body("Hello World");
}

