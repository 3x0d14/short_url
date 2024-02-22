#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

use crate::routes::{get_destination, shorten, test};
/// Utilities to create a Postgres db pool and communicate with the database
mod db;
/// Defines the models corresponding to our table schema
mod models;
/// Defines the routes of the application
mod routes;
/// Diesel schema
mod schema;
/// Various needed data structures
mod structs;

/// Main entry point for a Rocket application
#[launch]
fn rocket() -> _ {
    let url = dotenv!("DATABASE_URL");
    println!("{url}");
    let pool = db::init_pool(url.into()).expect("Failed");
    rocket::build()
        .manage(pool)
        .mount("/", routes![test, shorten, get_destination])
}
