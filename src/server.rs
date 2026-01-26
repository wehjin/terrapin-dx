use crate::data::market::Product;
use crate::data::portfolio::Lot;
use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
    pub products: Vec<Product>,
    pub lots: Vec<Lot>,
}

#[cfg(feature = "server")]
mod backend {
    use crate::data::market::Product;
    use crate::data::portfolio::Lot;
    use crate::data::{market, portfolio};
    use dioxus::fullstack::ServerFnError;
    use std::path::Path;

    pub fn read_products(data_path: &Path) -> Result<Vec<Product>, ServerFnError> {
        let path = data_path.join("products.csv");
        let bytes = std::fs::read(path)
            .map_err(|e| ServerFnError::new(format!("Failed to read products: {}", e)))?;
        let products = market::parse_products(&bytes)
            .map_err(|e| ServerFnError::new(format!("Failed to parse products: {}", e)))?;
        Ok(products)
    }

    pub fn read_lots(data_path: &Path) -> Result<Vec<Lot>, ServerFnError> {
        let path = data_path.join("lots.csv");
        let bytes = std::fs::read(path)
            .map_err(|e| ServerFnError::new(format!("Failed to read lots: {}", e)))?;
        let lots = portfolio::parse_lots(&bytes)
            .map_err(|e| ServerFnError::new(format!("Failed to parse lots: {}", e)))?;
        Ok(lots)
    }
}

#[server]
pub async fn load_session(login_name: String) -> Result<SessionState, ServerFnError> {
    use std::path::Path;

    let data_path = Path::new("data").join(login_name.clone());
    if !data_path.exists() {
        return Err(ServerFnError::new(format!(
            "Invalid login name: {}",
            login_name
        )));
    }
    let state = SessionState {
        login_name,
        products: backend::read_products(&data_path)?,
        lots: backend::read_lots(&data_path)?,
    };
    Ok(state)
}
