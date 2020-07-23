use actix_web::{web, App, HttpServer, Responder, get, Error, HttpResponse};
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::middleware::Logger;
use env_logger::Env;

mod config;
mod models;
mod actions;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    web::HttpResponse::Ok().body("OK")
}

#[get("/item/{id}")]
async fn get_item(pool: web::Data<DbPool>, id: web::Path<i64>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let item = web::block(move || actions::find_item(id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(item))
}

#[get("/items")]
async fn get_items(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let items = web::block(move || actions::find_items(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(items))
}


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
            .service(index)
            .service(get_item)
            .service(get_items)
    })
    .bind(address)?
    .run()
    .await
}
