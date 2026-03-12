use crate::api::session::SessionState;
use crate::backend;
use crate::api::ecs::Ecs;
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
    let ecs = Ecs::connect(&data_path)
        .map_err(|e| ServerFnError::new(format!("Failed to connect to ECS: {}", e)))?;
    let state = SessionState {
        login_name,
        products,
        ecs,
    };
    Ok(state)
}
