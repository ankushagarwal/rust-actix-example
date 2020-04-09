use crate::auth::get_identity_service;
use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use actix_web::{middleware::Logger, App, HttpServer};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

mod auth;
mod config;
mod database;
mod errors;
mod extractors;
pub mod handlers;
mod helpers;
mod middleware;
mod models;
mod routes;
mod schema;
mod state;
mod tests;
mod validate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in handlers using data: AppState<'_, String>
    let data = state::new_state::<String>();

    let mut server = HttpServer::new(move || {
        App::new()
          .wrap(Logger::default())
          .wrap(get_identity_service())
          .configure(add_pool)
          .app_data(data.clone())
          .configure(routes)
    });
    server = server.bind(&CONFIG.server)?;
    server.run().await
}
