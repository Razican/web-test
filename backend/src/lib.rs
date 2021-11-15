//! Backend of the MySupport application
//!
//! This crate defines the API, database glue and frontend glue of the MySupport application.

mod api;
mod db;
mod frontend;

#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket};

/// Initializes the rocket.
pub fn initialize() -> Rocket<Build> {
    rocket::build()
        .mount("/", frontend::routes())
        .mount("/api/v1", api::routes())
        .attach(db::Connection::fairing())
}
