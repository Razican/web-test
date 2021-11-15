use crate::sync_client;
use rocket::http::{ContentType, Status};

/// Sunny integration test for the `/api/v1/hello/<email>` endpoint for Alice.
#[test]
fn it_sunny_hello_alice() {
    let client = sync_client();
    let response = client.get("/api/v1/hello/alice@example.com").dispatch();

    assert_eq!(
        response.status(),
        Status::Ok,
        "response HTTP status code was not 200 OK"
    );
    assert_eq!(
        response.content_type(),
        Some(ContentType::Plain),
        "response content type was not plain text"
    );

    assert!(
        response
            .into_string()
            .expect("body was not a valid string")
            .contains("Alice"),
        "the response body didn't contain `Alice`"
    )
}

/// Sunny integration test for the `/api/v1/hello/<email>` endpoint for Bob.
#[test]
fn it_sunny_hello_bob() {
    let client = sync_client();
    let response = client.get("/api/v1/hello/bob@example.com").dispatch();

    assert_eq!(
        response.status(),
        Status::Ok,
        "response HTTP status code was not 200 OK"
    );
    assert_eq!(
        response.content_type(),
        Some(ContentType::Plain),
        "response content type was not plain text"
    );

    assert!(
        response
            .into_string()
            .expect("body was not a valid string")
            .contains("Bob"),
        "the response body didn't contain `Bob`"
    )
}

/// Rainy integration test for the `/api/v1/hello/<email>` endpoint for a nonexistant email.
#[test]
fn it_rainy_hello_nonexistant() {
    let client = sync_client();
    let response = client.get("/api/v1/hello/bademail@example.com").dispatch();

    assert_eq!(
        response.status(),
        Status::Ok,
        "response HTTP status code was not 200 OK"
    );
    assert_eq!(
        response.content_type(),
        Some(ContentType::Plain),
        "response content type was not plain text"
    );

    assert!(
        response
            .into_string()
            .expect("body was not a valid string")
            .contains("no user"),
        "the response body didn't contain `no user`"
    )
}

/// Rainy integration test for the `/api/v1/hello/<email>` endpoint for an invalid email.
#[test]
fn it_rainy_hello_invalid() {
    let client = sync_client();
    let response = client.get("/api/v1/hello/invalid!email").dispatch();

    assert_eq!(
        response.status(),
        Status::Ok,
        "response HTTP status code was not 200 OK"
    );
    assert_eq!(
        response.content_type(),
        Some(ContentType::Plain),
        "response content type was not plain text"
    );

    assert!(
        dbg!(response.into_string())
            .expect("body was not a valid string")
            .contains("no user"),
        "the response body didn't contain `no user`"
    )
}
