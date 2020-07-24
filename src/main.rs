use actix_web::{web, App, HttpServer, Responder, Error, HttpResponse};
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::Deserialize;

mod config;
mod models;
mod actions;

use crate::models::*;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

async fn index() -> impl Responder {
    web::HttpResponse::Ok().body("OK")
}

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

#[derive(Deserialize)]
struct FormItem {
    title: String
}

async fn post_item(pool: web::Data<DbPool>, list_id: web::Path<i64>, form: web::Form<FormItem>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || actions::create_item(list_id.into_inner(), &form.title, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(result))
}

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

async fn get_items_of_list(pool: web::Data<DbPool>, list_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let items: Vec<(List, Item)> = web::block(move || actions::find_items_of_list(list_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let result: Vec<Item> = items
        .into_iter()
        .map(|(_, item)| item)
        .collect::<Vec<Item>>();
    Ok(HttpResponse::Ok().json(result))
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
            .configure(routes)
    })
    .bind(address)?
    .run()
    .await
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/v1")
                .route("/items", web::get().to(get_items))
                .route("/item/{id}", web::get().to(get_item))
                .route("/list/{id}", web::get().to(get_items_of_list))
                .route("/list/{id}", web::post().to(post_item)),
            )
        .service(
            web::scope("/")
                .route("", web::get().to(index)),
        );
}