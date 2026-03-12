use crate::data::portfolio;
use crate::data::portfolio::Lot;
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
}

#[derive(Error, Debug)]
pub enum DropError {
    #[error("Write error: {0}")]
    WriteError(#[from] WriteLotsError),
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
    #[error("Read error: {0}")]
    ReadError(#[from] ReadLotsError),
}

impl Ecs {
    pub fn connect(data_path: impl AsRef<Path>) -> Result<Self, ConnectError> {
        let data_path = data_path.as_ref();
        let lots = read_lots(data_path)?;
        Ok(Self {
            data_path: data_path.to_owned(),
            lots,
        })
    }
}

#[derive(Error, Debug)]
pub enum ReadLotsError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] portfolio::ParseLotsError),
}

fn read_lots(data_path: &Path) -> Result<HashMap<Eid, Lot>, ReadLotsError> {
    let path = lots_data_path(data_path);
    let bytes = std::fs::read(path)?;
    let lots = portfolio::parse_lots(&bytes)?;
    Ok(lots)
}

fn lots_data_path(data_path: &Path) -> PathBuf {
    let path = data_path.join("lots.csv");
    path
}

#[derive(Error, Debug)]
pub enum WriteLotsError {
    #[error("Format error: {0}")]
    FormatError(#[from] portfolio::FormatLotsError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

fn write_lots(lots: HashMap<Eid, Lot>, data_path: &Path) -> Result<(), WriteLotsError> {
    let path = lots_data_path(data_path);
    let string = portfolio::format_lots(lots)?;
    std::fs::write(path, string.as_bytes())?;
    Ok(())
}
