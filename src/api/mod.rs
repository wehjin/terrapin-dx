use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod authentication;
pub mod registration;
pub mod session;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub user_id: Uuid,
}

#[server]
pub async fn active_user() -> Result<Option<User>, ServerFnError> {
    use tower_sessions::Session;

    let session: Session = FullstackContext::extract().await?;
    let user = session
        .get::<User>("user")
        .await
        .map_err(|_| ServerFnError::new("Failed to get user from session"))?;
    Ok(user)
}
