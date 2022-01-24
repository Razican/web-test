use crate::db::schema::{sys_email_registration, sys_user};
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

/// Insertable user.
#[derive(Debug, Clone, Insertable)]
#[table_name = "sys_user"]
pub struct NewUser<'n> {
    /// Wether the user is active or not.
    pub active: bool,
    /// The username of the user.
    pub username: &'n str,
    /// The email of the user.
    ///
    /// It is guaranteed to contain an `@` symbol.
    pub email: &'n str,
    /// The password hash of the user.
    pub password: &'n [u8],
    /// The first name(s) of the user.
    pub first_name: &'n str,
    /// The last name(s) of the user.
    pub last_name: &'n str,
}

/// Structure representing an email registration in the database.
#[derive(Debug, Clone, Queryable)]
pub struct EmailRegistration {
    /// Unique email registration code.
    pub code: String,
    /// Email for the registration.
    pub email: String,
    /// Creation date of the email registration.
    pub created_on: DateTime<Utc>,
}

/// Insertable email registration.
#[derive(Debug, Clone, Insertable)]
#[table_name = "sys_email_registration"]
pub struct NewEmailRegistration<'n> {
    /// Unique email registration code.
    pub code: &'n str,
    /// Email for the email registration.
    pub email: &'n str,
}
