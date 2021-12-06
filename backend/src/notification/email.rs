use lettre::{
    message::{IntoBody, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use once_cell::sync::Lazy;
use std::{env, io};

use crate::into_io_err;

static FROM_EMAIL: Lazy<Mailbox> = Lazy::new(|| {
    let from_str = env::var("FROM_EMAIL").expect("FROM_EMAIL environment variable not supplied");
    from_str.parse().expect("invalid FROM_EMAIL found")
});
static SMTP_HOST: Lazy<String> =
    Lazy::new(|| env::var("SMTP_HOST").expect("SMTP_HOST environment variable not supplied"));
static SMTP_USER: Lazy<String> =
    Lazy::new(|| env::var("SMTP_USER").expect("SMTP_USER environment variable not supplied"));
static SMTP_PASSWORD: Lazy<String> = Lazy::new(|| {
    env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD environment variable not supplied")
});

/// Sends an email to the given address, with the provided subject and body.
pub fn send<S, B>(to: &str, subject: S, body: B) -> io::Result<()>
where
    S: Into<String>,
    B: IntoBody,
{
    let email = Message::builder()
        .from(FROM_EMAIL.clone())
        .reply_to(FROM_EMAIL.clone())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .expect("couldn't create message");

    let creds = Credentials::new(SMTP_USER.to_owned(), SMTP_PASSWORD.to_owned());

    let mailer = SmtpTransport::relay(&*SMTP_HOST)
        .expect("invalid SMTP_HOST provided")
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(into_io_err)?;
    // TODO: logging

    Ok(())
}
