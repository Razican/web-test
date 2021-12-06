use super::*;
use std::env;

/// Helper function to stablish database connections.
fn establish_connection() -> PgConnection {
    let _ = dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("error connecting to {}: {}", database_url, e))
}

/// Sunny day unit test for the `get_with_email()` function.
#[test]
fn ut_sunny_get_with_email() {
    let mut conn = establish_connection();

    let user = get_with_email(&mut conn, "alice@example.com")
        .expect("error retrieving user from database");
    assert!(user.is_some(), "Alice was not in the database");
    assert_eq!(
        user.unwrap().email,
        "alice@example.com",
        "email for Bob doesn't match"
    );

    let user =
        get_with_email(&mut conn, "bob@example.com").expect("error retrieving user from database");
    assert!(user.is_some(), "Bob was not in the database");
    assert_eq!(
        user.unwrap().email,
        "bob@example.com",
        "email for Bob doesn't match"
    );
}

/// Rainy day unit test for the `get_with_email()` function.
#[test]
fn ut_rainy_user_get_with_email() {
    let mut conn = establish_connection();

    let user = get_with_email(&mut conn, "none@undefined.com")
        .expect("error retrieving user from database");
    assert!(
        user.is_none(),
        "some user was found with a nonexistant email"
    );

    let user =
        get_with_email(&mut conn, "not!an:email").expect("error retrieving user from database");
    assert!(
        user.is_none(),
        "some user was found with an incorrect email"
    );
}
