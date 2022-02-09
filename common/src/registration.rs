use serde::{Deserialize, Serialize};

/// Email registration form data.
#[derive(Debug, Serialize, Deserialize)]
pub struct Email<'r> {
    pub email: &'r str,
}

impl<'r> Email<'r> {
    /// Creates a new `Email` structure.
    pub fn new(email: &'r str) -> Self {
        Self { email }
    }
}

/// Data Transfer Object used from the client when transferring the registration form information
/// to the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitDTO<'d> {
    #[serde(rename = "user")]
    pub username: &'d str,
    #[serde(rename = "pass")]
    pub password: &'d str,
    #[serde(rename = "fn")]
    pub first_name: &'d str,
    #[serde(rename = "ln")]
    pub last_name: &'d str,
}

/// Data Transfer Object used from the server when transferring submission errors to the client.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResponseDTO {
    #[serde(rename = "user")]
    pub username: Option<String>,
    #[serde(rename = "pass")]
    pub password: Option<String>,
    pub other: Option<String>,
}

impl ResponseDTO {
    /// Checks if no errors have been set in the response
    pub fn is_ok(&self) -> bool {
        self.username.is_none() && self.password.is_none() && self.other.is_none()
    }
}
