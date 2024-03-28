use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use sea_orm::{Database, Entity};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod model;
mod routes;

pub async fn initialize_database() -> Result<(), sea_orm::DbErr> {
    let db = Database::connect_lazy().await?;
    db.migrate().run().await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect_lazy().await.expect("Failed to connect to database");

    db::initialize_database().await.expect("Failed to initialize database");

    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(routes::get_users))
            .route("/users", web::put().to(routes::create_user))
            .route("/users/{id}", web::get().to(routes::get_user_by_id))
            .route("/users/{id}", web::delete().to(routes::delete_user))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}