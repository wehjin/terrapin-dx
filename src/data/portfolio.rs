use crate::api::ecs::Eid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LotComponent {
    pub account: String,
    pub time: DateTime<Utc>,
    pub product: String,
    pub quantity: f64,
    pub eid: Eid,
}

pub fn parse_lots(csv_data: &[u8]) -> Result<HashMap<Eid, Lot>, LotReadError> {
    let mut reader = csv::ReaderBuilder::new().from_reader(csv_data);
    let vec: Vec<LotComponent> = reader
        .deserialize()
        .collect::<Result<Vec<LotComponent>, _>>()?;
    let map = vec
        .into_iter()
        .map(|c| {
            let lot = Lot {
                account: c.account,
                time: c.time,
                product: c.product,
                quantity: c.quantity,
            };
            (c.eid, lot)
        })
        .collect();
    Ok(map)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lot {
    pub account: String,
    pub time: DateTime<Utc>,
    pub product: String,
    pub quantity: f64,
}

#[derive(Error, Debug)]
pub enum LotReadError {
    #[error("Csv read error: {0}")]
    CsvReadError(#[from] csv::Error),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_lots() {
        let csv_data =
            "account,time,product,quantity,eid\nfoo,2021-01-01T00:00:00Z,AAPL,100,l0".as_bytes();

        let lots = super::parse_lots(csv_data).unwrap();
        assert_eq!(lots.len(), 1);
    }
}
