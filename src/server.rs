use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
}

#[server]
pub async fn load_session(login_name: String) -> Result<SessionState, ServerFnError> {
    use std::path::Path;
    let path = Path::new("data").join(login_name.clone());
    if path.exists() {
        Ok(SessionState { login_name })
    } else {
        Err(ServerFnError::new("Invalid login name"))
    }
}
