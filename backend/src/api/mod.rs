use crate::db;
use rocket::{get, routes, Route};
use std::io;

/// Gets the routes for the backend API.
pub fn routes() -> Vec<Route> {
    routes![hello]
}

/// Hello world
#[get("/hello/<email>")]
pub async fn hello(conn: db::Connection, email: String) -> io::Result<String> {
    let user = conn
        .run(move |c| db::retrieve_user_with_email(c, &email))
        .await?;

    if let Some(user) = user {
        Ok(format!("Hello {}!", user.first_name))
    } else {
        Ok("There is no user with this email :(".to_owned())
    }
}
