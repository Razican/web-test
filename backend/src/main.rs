//! Backend of the MySupport application
//!
//! This crate defines the API, database glue and frontend glue of the MySupport application.
//! It also includes the point of entry for launching it.
mod api;
mod db;
mod frontend;

#[macro_use]
extern crate diesel;

/// Entry point of the backend
///
/// This function will configure the Rocket framework and launch it. It is
/// equivalent to a `main()` function thanks to the `#[launch]` attribute.
#[rocket::launch]
fn launch() -> _ {
    let _ = dotenv::dotenv().ok();

    rocket::build()
        .mount("/", frontend::routes())
        .mount("/api/v1", api::routes())
        .attach(db::Connection::fairing())
}
