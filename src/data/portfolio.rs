use crate::api::ecs::Eid;
use crate::data::{FormatError, ParseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lot {
    pub account: String,
    pub time: DateTime<Utc>,
    pub product: String,
    pub quantity: f64,
}

pub fn parse_lots(csv_data: &[u8]) -> Result<HashMap<Eid, Lot>, ParseError> {
    let mut reader = csv::ReaderBuilder::new().from_reader(csv_data);
    let rows: Vec<LotCsvRow> = reader
        .deserialize()
        .collect::<Result<Vec<LotCsvRow>, _>>()?;
    let map = rows
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

pub fn format_lots(lots: HashMap<Eid, Lot>) -> Result<String, FormatError> {
    let rows = lots
        .into_iter()
        .map(|(eid, lot)| LotCsvRow {
            account: lot.account,
            time: lot.time,
            product: lot.product,
            quantity: lot.quantity,
            eid,
        })
        .collect::<Vec<_>>();
    let mut writer = csv::WriterBuilder::new().from_writer(vec![]);
    for row in rows {
        writer.serialize(row)?;
    }
    let data = writer.into_inner()?;
    let string = String::from_utf8(data)?;
    Ok(string)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LotCsvRow {
    pub account: String,
    pub time: DateTime<Utc>,
    pub product: String,
    pub quantity: f64,
    pub eid: Eid,
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
