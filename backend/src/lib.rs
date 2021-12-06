//! Backend of the MySupport application
//!
//! This crate defines the API, database glue and frontend glue of the MySupport application.

mod api;
mod db;
mod frontend;
mod notification;

#[macro_use]
extern crate diesel;

use once_cell::sync::Lazy;
use rocket::{Build, Rocket};
use std::{env, error::Error, io};

static BASE_URL: Lazy<String> =
    Lazy::new(|| env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_owned()));

/// Initializes the rocket.
pub fn initialize() -> Rocket<Build> {
    rocket::build()
        .mount("/", frontend::routes())
        .mount("/api/v1", api::routes())
        .attach(db::Connection::fairing())
}

/// Converts any error into an I/O error.
fn into_io_err<E>(err: E) -> io::Error
where
    E: Into<Box<dyn Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}
