use crate::data::market::Product;
use crate::data::portfolio::Lot;
use crate::data::{market, portfolio};
use dioxus::fullstack::ServerFnError;
use std::path::{Path, PathBuf};

pub mod passkey;

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

pub fn user_data_path(user: impl AsRef<str>) -> PathBuf {
    PathBuf::from("data").join(user.as_ref())
}
