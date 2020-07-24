use actix_web::{App, HttpServer};
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::middleware::Logger;
use env_logger::Env;

mod config;
mod models;
mod actions;
mod handlers;
mod routes;

use crate::routes::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();
    let conf = config::Config::from_env().unwrap();
    let address = format!("{}:{}", conf.server.host, conf.server.port);
    let database_url = conf.database_url;

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Lestening for http://{}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(routes)
    })
    .bind(address)?
    .run()
    .await
}