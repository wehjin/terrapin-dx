use crate::api::ecs::Ecs;
use crate::api::{active_user, User};
use dioxus::fullstack::ServerFnError;
use std::path::PathBuf;

pub mod passkey;
pub mod session;

pub fn user_data_path(user: impl AsRef<str>) -> PathBuf {
    PathBuf::from("data").join(user.as_ref())
}

pub async fn require_ecs() -> dioxus::Result<Ecs, ServerFnError> {
    let user = require_user().await?;
    let data_path = user_data_path(&user.username);
    Ecs::connect(&data_path)
        .map_err(|e| ServerFnError::new(format!("Failed to connect to ECS: {}", e)))
}

pub async fn require_user() -> dioxus::Result<User, ServerFnError> {
    active_user().await?.ok_or(ServerFnError::new("No user"))
}
