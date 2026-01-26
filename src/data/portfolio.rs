use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub fn parse_lots(csv_data: &[u8]) -> Result<Vec<Lot>, LotReadError> {
    let mut reader = csv::ReaderBuilder::new().from_reader(csv_data);
    let records: Vec<Lot> = reader.deserialize().collect::<Result<Vec<Lot>, _>>()?;
    Ok(records)
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
            "account,time,product,quantity\nfoo,2021-01-01T00:00:00Z,AAPL,100".as_bytes();

        let lots = super::parse_lots(csv_data).unwrap();
        assert_eq!(lots.len(), 1);
    }
}
