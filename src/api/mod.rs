use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use session::SessionState;
use uuid::Uuid;

pub mod authentication;
pub mod registration;
pub mod session;

#[server]
pub async fn load_session(login_name: String) -> Result<SessionState, ServerFnError> {
    use crate::backend;

    let data_path = backend::user_data_path(&login_name);
    if false == data_path.exists() {
        return Err(ServerFnError::new(format!(
            "Invalid login name: {}",
            login_name
        )));
    }
    let products = backend::read_products(&data_path)?;
    let state = SessionState {
        login_name,
        products,
        lots: backend::read_lots(&data_path)?,
    };
    Ok(state)
}

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
