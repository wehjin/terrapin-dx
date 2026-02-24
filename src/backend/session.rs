use crate::api::session::SessionState;
use crate::backend;
use dioxus::prelude::*;

pub async fn load_session(login_name: String) -> Result<SessionState, ServerFnError> {
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
