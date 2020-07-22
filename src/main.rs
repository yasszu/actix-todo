mod config;
mod models;

use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let conf = config::Config::from_env().unwrap();
    let address = format!("{}:{}", conf.server.host, conf.server.port);

    println!("Lestening for http://{}", address);

    HttpServer::new(|| 
        App::new().route("/", web::get().to(status))
    )
    .bind(address)?
    .run()
    .await
}
