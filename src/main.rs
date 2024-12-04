use actix_web::{App, HttpServer};

mod models;
mod routes;
mod controller;
mod utils;
pub mod casdoor;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    //rust环境变量配置
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await

}

