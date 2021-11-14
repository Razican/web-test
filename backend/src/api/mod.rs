use rocket::{get, routes, Route};

/// Gets the routes for the backend API.
pub fn routes() -> Vec<Route> {
    routes![hello]
}

/// Hello world
#[get("/hello/<name>")]
pub async fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}
