use crate::data::market::Product;
use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
    pub products: HashMap<String, Product>,
}

#[server]
pub async fn load_session(login_name: String) -> Result<SessionState, ServerFnError> {
    use crate::data::market;
    use std::path::Path;

    let data_path = Path::new("data").join(login_name.clone());
    if !data_path.exists() {
        return Err(ServerFnError::new(format!("Invalid login name: {}", login_name )));
    }
    let products = {
        let path = data_path.join("products.csv");
        let bytes = std::fs::read(path)
            .map_err(|e| ServerFnError::new(format!("Failed to read products: {}", e)))?;
        dbg!(bytes.len());

        let items = market::parse_products(&bytes)
            .map_err(|e| ServerFnError::new(format!("Failed to parse products: {}", e)))?;
        items
            .into_iter()
            .map(|it| (it.symbol().to_string(), it))
            .collect::<HashMap<_, _>>()
    };
    let state = SessionState {
        login_name,
        products,
    };
    Ok(state)
}
