use crate::{db, notification, BASE_URL};
use common::registration::{Email, ResponseDTO, SubmitDTO};
use once_cell::sync::Lazy;
use rand::{distributions, thread_rng, Rng, RngCore};
use regex::Regex;
use rocket::{http::Status, post, serde::json::Json, tokio::task::spawn_blocking};
use sha3::{Digest, Sha3_256};
use std::{io, sync::Arc};
use zxcvbn::{zxcvbn, ZxcvbnError};

static VALID_EMAIL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([^@]+)@([^@]+\.[^@]+)")
        .expect("could not compile the valid email regular expression")
});

const FORBIDDEN_DOMAINS: [&str; 15] = [
    "mailinator.com",
    "vusra.com",
    "tormails.com",
    "ecodaw.com",
    "pp7rvv.com",
    "jmortgageli.com",
    "allfreemail.net",
    "incorporatedmail.com",
    "ultramailinator.com",
    "appmailer.org",
    "basicmail.host",
    "easymailer.live",
    "easyonlinemail.net",
    "freemailonline.us",
    "example.com",
];

/// Register a given email
#[post("/register/email", format = "json", data = "<email>")]
pub async fn email(
    conn: db::Connection,
    email: Json<Email<'_>>,
) -> io::Result<(Status, Json<&'static str>)> {
    let email = Arc::new(email.into_inner().email.trim().to_owned());

    // Check if the email is correct:
    if !VALID_EMAIL.is_match(&email) {
        return Ok((Status::BadRequest, Json("invalid email")));
    }
    let cap = VALID_EMAIL
        .captures(&email)
        .expect("couldn't get captures for email address after validation");

    let domain = cap
        .get(2)
        .expect("email domain disappeared")
        .as_str()
        .to_lowercase();

    if FORBIDDEN_DOMAINS
        .into_iter()
        .any(|forbidden| forbidden == domain)
    {
        return Ok((Status::BadRequest, Json("email not allowed")));
    }

    // Checks if there was an existing user with the email
    let email_clone = email.clone();
    if conn
        .run(move |c| db::user::get_with_email(c, &email_clone))
        .await?
        .is_some()
    {
        // You don't want to give information about the existence of the user in the DB
        return Ok((Status::Ok, Json("")));
    }

    // TODO: check if email registered recently (do not send more than one per 10 minutes)
    // Remove any existing codes for that email
    let email_clone = email.clone();
    conn.run(move |c| db::user::delete_email_registrations_for_email(c, &email_clone))
        .await?;

    // Generate the random code
    let code = loop {
        let code = Arc::new(rand_reg_code());
        let code_clone = code.clone();
        if conn
            .run(move |c| db::user::get_email_registration_with_code(c, &code_clone))
            .await?
            .is_none()
        {
            break code;
        }
    };

    let email_clone = email.clone();
    let code_clone = code.clone();
    let _ = conn
        .run(move |c| db::user::insert_email_registration(c, &email_clone, &code_clone))
        .await?;

    #[cfg(debug_assertions)]
    println!("Registration code: {}", code);

    let body = format!(
        "Welcome to MySupport!
        
        In order to register your account, please follow {}/register/{}
        
        Best regards,
        The MySupport team",
        *BASE_URL, code
    );

    // TODO: use queues
    spawn_blocking(move || notification::email::send(&email, "Registration in MySupport", body))
        .await??;

    Ok((Status::Ok, Json("")))
}

/// Creates a random registration code.
fn rand_reg_code() -> String {
    let vec = thread_rng()
        .sample_iter(distributions::Alphanumeric)
        .take(10)
        .collect::<Vec<u8>>();

    // We know that the code is ASCII
    String::from_utf8(vec).expect("invalid code generated")
}

/// Register a user from a given code
#[post("/register/user/<code>", format = "json", data = "<user>")]
pub async fn register(
    conn: db::Connection,
    code: String,
    user: Json<SubmitDTO<'_>>,
) -> io::Result<(Status, Json<ResponseDTO>)> {
    let user = user.into_inner();
    let mut response = ResponseDTO::default();

    let email_registration = conn
        .run(move |c| db::user::get_email_registration_with_code(c, &code))
        .await?;

    let email = if let Some(reg) = email_registration {
        Arc::new(reg.email)
    } else {
        response.set_other("invalid registration code");
        return Ok((Status::BadRequest, Json(response)));
    };

    let cloned_email = email.clone();
    let db_user = conn
        .run(move |c| db::user::get_with_email(c, &cloned_email))
        .await?;

    if db_user.is_some() {
        response.set_username("user already exists");
    }

    match zxcvbn(
        user.password,
        &[&email, user.username, user.first_name, user.last_name],
    ) {
        Ok(entropy) => {
            if entropy.score() < 3 {
                response.set_password("password entropy is too low");
            }
        }
        Err(e) => match e {
            ZxcvbnError::BlankPassword => {
                response.set_password("blank password not allowed");
            }
            ZxcvbnError::DurationOutOfRange => {
                return Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }
        },
    };

    if response.is_ok() {
        let mut salt = [0u8; 10];
        thread_rng().fill_bytes(&mut salt); // Salt

        let mut hasher = Sha3_256::new();
        hasher.update(&salt);
        hasher.update(user.password.as_bytes());

        let result = hasher.finalize();
        let mut db_pass = Vec::with_capacity(42);
        db_pass.extend_from_slice(&salt);
        db_pass.extend_from_slice(&result[..]);

        debug_assert_eq!(db_pass.len(), 42);

        let (username, password, first_name, last_name) = (
            user.username.to_owned(),
            db_pass,
            user.first_name.to_owned(),
            user.last_name.to_owned(),
        );

        conn.run(move |c| {
            db::user::insert_user(c, &username, &email, &password, &first_name, &last_name)
        })
        .await?;

        Ok((Status::Ok, Json(response)))
    } else {
        Ok((Status::BadRequest, Json(response)))
    }
}
