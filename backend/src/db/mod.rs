//! Database glue for the MySupport backend.
//!
//! This module includes the models and schema for the MySupport application,
//! along with helper functions to manipulate the required data.

pub mod model;
mod schema;
pub mod user;

use std::io;

use diesel::{PgConnection, QueryResult};
use rocket_sync_db_pools::database;

/// PostgreSQL database connection.
#[database("main")]
pub struct Connection(PgConnection);

/// Converts a DB result into an optional result.
fn into_option<T>(res: QueryResult<T>) -> io::Result<Option<T>> {
    match res {
        Ok(val) => Ok(Some(val)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}
