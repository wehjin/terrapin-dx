use crate::api::ecs::{Eid, LotItem};
use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod authentication;
pub mod ecs;
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

#[server]
pub async fn query_lots() -> Result<Vec<LotItem>, ServerFnError> {
    use crate::api::session::fetch_session;
    let session = fetch_session()
        .await?
        .ok_or(ServerFnError::new("No session"))?;
    Ok(session.ecs.query_lots())
}

#[server]
pub async fn drop_lot(eid: Eid) -> Result<(), ServerFnError> {
    use crate::api::session::fetch_session;
    let mut session = fetch_session()
        .await?
        .ok_or(ServerFnError::new("No session"))?;
    session
        .ecs
        .drop_lot(eid.clone())
        .map_err(|_| ServerFnError::new("Failed to drop lot"))?;
    info!("Dropped lot: {:?}", eid);
    Ok(())
}
