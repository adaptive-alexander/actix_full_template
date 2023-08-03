mod config;
mod db;
mod error;
mod logging;
pub mod prelude;
mod tests;
mod handlers;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use clap::Parser;
use std::net::TcpListener;
use tracing::log::info;
use crate::prelude::*;

pub fn sync_test(_inp: &str) {}

#[tracing::instrument(name = "Async test subscriber")]
pub async fn async_test(_inp: usize) {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

pub async fn run(listener: TcpListener) -> std::io::Result<Server> {
    // Tracing, used for instrumenting spans
    let subscriber = get_subscriber("template".into(), "info".into());
    init_subscriber(subscriber);

    // Read .env file
    dotenvy::dotenv().expect("Failed to read .env file");

    // Read config.toml
    let _config = read_config().expect("Failed reading config");

    // Clap and example info log
    tracing::info!("Parsing args");
    let _args: Args = Args::parse();

    // Database connection
    let db_pool = get_db_pool().await.expect("Failed getting database pool");

    Ok(HttpServer::new(move || {
        App::new()
            // Register database connection as app data, this lets us get access to the pool via extractors
            .app_data(Data::new(db_pool.clone()))
            // Configure path handlers, add handler services through builder pattern
            .configure(handlers::register)
            // CORS policies, open for development. Restrict for production!
            .wrap(
                Cors::default()
                    .allow_any_origin() // Only for development!
                    .allow_any_header(),
            ) // Only for development!
            // Register logger
            .wrap(Logger::default()) // Needed for middleware to log incoming requests
    })
    // Can be set higher if more resources are available. It unset defaults to nr of cores.
    .workers(2)
    .listen(listener)? // Automatically bind to localhost:$port
    .run())
}
