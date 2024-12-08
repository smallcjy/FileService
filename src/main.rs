use actix_web::{web, App, HttpServer};

mod routes;
mod controller;
mod utils;
pub mod casdoor;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    //rust环境变量配置
    dotenv::dotenv().ok();
    env_logger::init();

    // register s3 client
    let s3_client = controller::s3::s3_client().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(s3_client.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}

