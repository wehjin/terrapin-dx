use crate::api::ecs::Ecs;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
    pub ecs: Ecs,
}
#[server]
pub async fn fetch_session() -> Result<Option<SessionState>, ServerFnError> {
    use crate::api::active_user;
    use crate::backend::session::load_session;

    match active_user().await? {
        None => Ok(None),
        Some(user) => {
            let session = load_session(user.username).await?;
            Ok(Some(session))
        }
    }
}
