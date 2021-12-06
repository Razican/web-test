use crate::db::schema::sys_email_registration;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Structure representing a user in the database.
#[derive(Debug, Clone, Queryable)]
pub struct User {
    /// The ID of the user.
    pub id: Uuid,
    /// Wether the user is active or not.
    pub active: bool,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    ///
    /// It is guaranteed to contain an `@` symbol.
    pub email: String,
    /// The password hash of the user.
    pub password: Vec<u8>,
    /// The first name(s) of the user.
    pub first_name: String,
    /// The last name(s) of the user.
    pub last_name: String,
    /// The timestamp for the creation of the user.
    pub created_on: DateTime<Utc>,
    /// The timestamp for the last update of the user record.
    pub updated_on: DateTime<Utc>,
}

/// Structure representing an email registration in the database.
#[derive(Debug, Clone, Queryable)]
pub struct EmailRegistration {
    pub code: String,
    pub email: String,
    pub created_on: DateTime<Utc>,
}

/// Insertable email registration.
#[derive(Debug, Clone, Insertable)]
#[table_name = "sys_email_registration"]
pub struct NewEmailRegistration<'n> {
    pub code: &'n str,
    pub email: &'n str,
}
