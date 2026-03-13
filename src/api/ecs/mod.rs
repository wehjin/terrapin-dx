use crate::data;
use crate::data::market::Product;
use crate::data::portfolio::Lot;
use crate::data::yf::MarketPrice;
use crate::data::{market, portfolio};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Eid(String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LotItem(pub Lot, pub Eid);

impl LotItem {
    pub fn to_eid(&self) -> Eid {
        self.1.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ecs {
    pub data_path: PathBuf,
    pub lots: HashMap<Eid, Lot>,
    pub products: Vec<Product>,
}

#[derive(Error, Debug)]
pub enum DropError {
    #[error("Write error: {0}")]
    WriteError(#[from] WriteError),
}

impl Ecs {
    pub fn query_products(&self) -> Vec<Product> {
        self.products.clone()
    }
    pub fn update_prices(&mut self, prices: impl AsRef<[MarketPrice]>) -> Result<(), DropError> {
        let prices = prices.as_ref();
        let prices = prices
            .iter()
            .map(|p| (p.symbol.clone(), p.share_price.clone()))
            .collect::<HashMap<_, _>>();
        let mut products = self.products.clone();
        for product in products.iter_mut() {
            if let Some(price) = prices.get(product.symbol()) {
                product.set_share_price(price.clone());
            }
        }
        write_products(products.clone(), &self.data_path)?;
        self.products = products;
        Ok(())
    }
}

impl Ecs {
    pub fn lots(&self) -> Vec<Lot> {
        self.lots.values().cloned().collect()
    }
    pub fn query_lots(&self) -> Vec<LotItem> {
        self.lots
            .iter()
            .map(|(eid, lot)| LotItem(lot.clone(), eid.clone()))
            .collect()
    }
    pub fn drop_lot(&mut self, eid: Eid) -> Result<(), DropError> {
        let mut lots = self.lots.clone();
        lots.remove(&eid);
        write_lots(lots.clone(), &self.data_path)?;
        self.lots = lots;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("Read lots error: {0}")]
    ReadError(#[from] ReadError),
}

impl Ecs {
    pub fn connect(data_path: impl AsRef<Path>) -> Result<Self, ConnectError> {
        let data_path = data_path.as_ref();
        let lots = read_lots(data_path)?;
        let products = read_products(data_path)?;
        Ok(Self {
            data_path: data_path.to_owned(),
            lots,
            products,
        })
    }
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Portfolio parse error: {0}")]
    DataParseError(#[from] data::ParseError),
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Format error: {0}")]
    FormatError(#[from] data::FormatError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

fn read_products(data_path: &Path) -> Result<Vec<Product>, ReadError> {
    let path = products_csv_path(data_path);
    let bytes = std::fs::read(path)?;
    let products = market::parse_products(&bytes)?;
    Ok(products)
}

fn write_products(products: Vec<Product>, data_path: &Path) -> Result<(), WriteError> {
    let path = products_csv_path(data_path);
    let string = market::format_products(products)?;
    std::fs::write(path, string.as_bytes())?;
    Ok(())
}

fn read_lots(data_path: &Path) -> Result<HashMap<Eid, Lot>, ReadError> {
    let path = lots_csv_path(data_path);
    let bytes = std::fs::read(path)?;
    let lots = portfolio::parse_lots(&bytes)?;
    Ok(lots)
}

fn write_lots(lots: HashMap<Eid, Lot>, data_path: &Path) -> Result<(), WriteError> {
    let path = lots_csv_path(data_path);
    let string = portfolio::format_lots(lots)?;
    std::fs::write(path, string.as_bytes())?;
    Ok(())
}

fn products_csv_path(data_path: &Path) -> PathBuf {
    let path = data_path.join("products.csv");
    path
}

fn lots_csv_path(data_path: &Path) -> PathBuf {
    let path = data_path.join("lots.csv");
    path
}
