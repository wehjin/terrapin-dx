use serde::{Deserialize, Serialize};
use crate::data::market::Product;
use crate::data::portfolio::Lot;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
    pub products: Vec<Product>,
    pub lots: Vec<Lot>,
}