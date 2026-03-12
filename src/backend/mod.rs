use crate::data::market;
use crate::data::market::Product;
use dioxus::fullstack::ServerFnError;
use std::path::{Path, PathBuf};

pub mod passkey;
pub mod session;

pub fn read_products(data_path: &Path) -> Result<Vec<Product>, ServerFnError> {
    let path = data_path.join("products.csv");
    let bytes = std::fs::read(path)
        .map_err(|e| ServerFnError::new(format!("Failed to read products: {}", e)))?;
    let products = market::parse_products(&bytes)
        .map_err(|e| ServerFnError::new(format!("Failed to parse products: {}", e)))?;
    Ok(products)
}

pub fn user_data_path(user: impl AsRef<str>) -> PathBuf {
    PathBuf::from("data").join(user.as_ref())
}
