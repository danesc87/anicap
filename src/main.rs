use actix_web::{
    HttpServer,
    App,
    middleware::Logger
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

mod config;
mod utils;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Put log type as env variable since env_logger uses it
    std::env::set_var("RUST_LOG", "");
    // Init env_logger
    env_logger::init();

    let server_url = format!(
        "{}:{}",
        "127.0.0.1",
        "8085"
    );
    println!("\nServer running on :{}\n", server_url);

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        // .wrap() // Cors
        // .data() //Database
        // .config() // Routes
    })
    .bind(server_url)?
    .run()
    .await
}
