use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use session::SessionState;

pub mod auth;
pub mod session;

#[server]
pub async fn load_session(login_name: String) -> Result<SessionState, ServerFnError> {
    use crate::backend;
    use std::path::Path;

    let data_path = Path::new("data").join(login_name.clone());
    if !data_path.exists() {
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
