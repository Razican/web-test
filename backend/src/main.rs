//! Backend of the MySupport application
//!
//! This crate contains the entry point of the backend.

/// Entry point of the backend
///
/// This function will configure the Rocket framework and launch it. It is
/// equivalent to a `main()` function thanks to the `#[launch]` attribute.
#[rocket::launch]
fn launch() -> _ {
    let _ = dotenv::dotenv().ok();

    backend_core::initialize()
}
