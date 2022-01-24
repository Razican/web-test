use super::{into_option, model, schema::*};
use crate::into_io_err;
use chrono::{Duration, Utc};
use diesel::{prelude::*, PgConnection};
use std::io;

#[cfg(test)]
mod tests;

/// Timeout for email registration codes, in seconds.
const EMAIL_CODE_TIMEOUT: i64 = 2 * 60 * 60;

/// Retrieves a user with an email, if it exists.
pub fn get_with_email(conn: &mut PgConnection, email: &str) -> io::Result<Option<model::User>> {
    let user = sys_user::table
        .filter(sys_user::email.eq(email))
        .first(conn);

    into_option(user)
}

/// Inserts a new user into the database.
pub fn insert_user(
    conn: &mut PgConnection,
    username: &str,
    email: &str,
    password: &[u8],
    first_name: &str,
    last_name: &str,
) -> io::Result<()> {
    let new_record = model::NewUser {
        active: true,
        username,
        email,
        password,
        first_name,
        last_name,
    };

    diesel::insert_into(sys_user::table)
        .values(&new_record)
        .execute(conn)
        .map(|_count| ())
        .map_err(into_io_err)
}

/// Retrieves a registration email with a given code, if it exists.
pub fn get_email_registration_with_code(
    conn: &mut PgConnection,
    code: &str,
) -> io::Result<Option<model::EmailRegistration>> {
    let now = Utc::now();
    let timeout = Duration::seconds(EMAIL_CODE_TIMEOUT);
    let ten_mins_ago = now - timeout;

    let email_reg = sys_email_registration::table
        .filter(
            sys_email_registration::code
                .eq(code)
                .and(sys_email_registration::created_on.ge(ten_mins_ago)),
        )
        .first::<model::EmailRegistration>(conn);

    into_option(email_reg)
}

/// Inserts the new email inthe registration list.
pub fn insert_email_registration(
    conn: &mut PgConnection,
    email: &str,
    code: &str,
) -> io::Result<()> {
    let new_record = model::NewEmailRegistration { email, code };
    diesel::insert_into(sys_email_registration::table)
        .values(&new_record)
        .execute(conn)
        .map(|_count| ())
        .map_err(into_io_err)
}

/// Deletes all email registrations for the given email.
pub fn delete_email_registrations_for_email(
    conn: &mut PgConnection,
    email: &str,
) -> io::Result<()> {
    diesel::delete(sys_email_registration::table.filter(sys_email_registration::email.eq(email)))
        .execute(conn)
        .map(|_count| ())
        .map_err(into_io_err)
}

/// Cleans up old email registrations.
#[allow(dead_code)]
pub fn cleanup_old_email_registrations(conn: &mut PgConnection) -> io::Result<()> {
    let now = Utc::now();
    let timeout = Duration::seconds(EMAIL_CODE_TIMEOUT);
    let ten_mins_ago = now - timeout;

    diesel::delete(
        sys_email_registration::table.filter(sys_email_registration::created_on.lt(ten_mins_ago)),
    )
    .execute(conn)
    .map(|_count| ())
    .map_err(into_io_err)
}
