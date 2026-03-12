use crate::data::portfolio;
use crate::data::portfolio::Lot;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Eid(String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ecs {
    pub lots: HashMap<Eid, Lot>,
}

impl Ecs {
    pub fn lots(&self) -> Vec<Lot> {
        self.lots.values().cloned().collect()
    }
}

#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),
}

impl Ecs {
    pub fn connect(data_path: impl AsRef<Path>) -> Result<Self, ConnectError> {
        let data_path = data_path.as_ref();
        let lots = read_lots(data_path)?;
        Ok(Self { lots })
    }
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Lot read error: {0}")]
    LotReadError(#[from] portfolio::LotReadError),
}

fn read_lots(data_path: &Path) -> Result<HashMap<Eid, Lot>, ReadError> {
    let path = data_path.join("lots.csv");
    let bytes = std::fs::read(path)?;
    let lots = portfolio::parse_lots(&bytes)?;
    Ok(lots)
}
