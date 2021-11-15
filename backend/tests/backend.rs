//! This crate contains the integration tests for the backend.

mod api;

use rocket::local::blocking::Client;

/// Creates a testing client for Rocket.
fn sync_client() -> Client {
    let _ = dotenv::dotenv().ok();

    let rocket = backend_core::initialize();
    Client::tracked(rocket).expect("couldn't generate the Rocket client")
}
