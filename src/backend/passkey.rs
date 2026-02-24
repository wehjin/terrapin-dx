use crate::backend::user_data_path;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::LazyLock;
use thiserror::Error;
use webauthn_rs::prelude::*;

pub static WEBAUTHN: LazyLock<Webauthn> = LazyLock::new(|| {
    let rp_id = std::env::var("WEBAUTHN_RP_ID").unwrap_or_else(|_| "localhost".to_string());
    let rp_origin_url =
        std::env::var("WEBAUTHN_RP_ORIGIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let rp_origin = url::Url::parse(&rp_origin_url).expect("Invalid RP Origin URL");
    WebauthnBuilder::new(&rp_id, &rp_origin)
        .expect("Invalid RP Configuration")
        .build()
        .expect("Failed to build Webauthn")
});

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Invalid username")]
    InvalidUsername,
    #[error("Username already exists: {0}")]
    DuplicateUsername(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub passkey: Passkey,
    pub user_id: Uuid,
    pub username: String,
}

pub fn register_user(login: Login) -> Result<(), RegistrationError> {
    // Check username
    let username = &login.username;
    if !is_valid_username(&username) {
        return Err(RegistrationError::InvalidUsername);
    }
    if is_active_user(&username) {
        return Err(RegistrationError::DuplicateUsername(username.clone()));
    }
    // Construct user directory
    let user_path = user_data_path(&username);
    fs::create_dir_all(&user_path)?;
    fs::write(user_path.join(LOGIN_FILE), serde_json::to_string(&login)?)?;
    Ok(())
}

pub fn load_login(username: &str) -> Result<Login, RegistrationError> {
    let user_path = user_data_path(username);
    let login_json = fs::read_to_string(user_path.join("login.json"))?;
    Ok(serde_json::from_str(&login_json)?)
}

pub fn is_active_user(username: &str) -> bool {
    user_data_path(username).join(LOGIN_FILE).exists()
}

pub fn is_valid_username(username: &str) -> bool {
    if username.is_empty() || !username.chars().next().unwrap().is_ascii_alphabetic() {
        return false;
    }

    username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationContext {
    pub passkey_authentication: PasskeyAuthentication,
    pub username: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationContext {
    pub passkey_registration: PasskeyRegistration,
    pub user_id: Uuid,
    pub username: String,
}

const LOGIN_FILE: &'static str = "login.json";
