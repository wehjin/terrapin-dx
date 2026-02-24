use crate::data::market::Product;
use crate::data::portfolio::Lot;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
    pub products: Vec<Product>,
    pub lots: Vec<Lot>,
}

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
