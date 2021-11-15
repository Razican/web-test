//! Database glue for the MySupport backend.
//!
//! This module includes the models and schema for the MySupport application,
//! along with helper functions to manipulate the required data.

pub mod model;
mod schema;
#[cfg(test)]
mod tests;

use diesel::{prelude::*, PgConnection};
use rocket_sync_db_pools::database;
use schema::*;
use std::io;

/// PostgreSQL database connection.
#[database("main")]
pub struct Connection(PgConnection);

/// Retrieves a user with an email, if it exists.
pub fn retrieve_user_with_email(
    conn: &mut PgConnection,
    email: &str,
) -> io::Result<Option<model::User>> {
    let user = sys_user::table
        .filter(sys_user::email.eq(email))
        .first(conn);

    match user {
        Ok(user) => Ok(Some(user)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}
