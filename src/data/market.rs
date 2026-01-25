use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct SharePrice {
    #[serde(rename = "share_price")]
    pub height: f64,
    #[serde(rename = "share_price_as_of")]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Product {
    Stock {
        symbol: String,
        name: String,
        outstanding_shares: usize,
        #[serde(flatten)]
        share_price: SharePrice,
    },
}

#[derive(Error, Debug)]
pub enum ProductReadError {
    #[error("Failed to read products from CSV file")]
    CsvReadError(#[from] csv::Error),
}
pub fn parse_products(csv_data: &[u8]) -> Result<Vec<Product>, ProductReadError> {
    #[derive(Debug, Deserialize)]
    struct ProductProxy {
        #[serde(rename = "type")]
        _product_type: String,
        #[serde(flatten)]
        product: Product,
    }
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(csv_data);
    let records: Vec<Product> = reader
        .deserialize::<ProductProxy>()
        .map(|result| result.map(|proxy| proxy.product))
        .collect::<Result<Vec<Product>, _>>()?;
    Ok(records)
}

#[cfg(test)]
mod tests {
    use crate::data::market::Product;
    use chrono::TimeZone;

    #[test]
    fn test_parse_products() {
        let csv_data = "type,symbol,name,outstanding_shares,share_price,share_price_as_of\nstock,AAPL,Apple Inc.,100,123.45,2021-01-01T00:00:00Z".as_bytes();
        let mut products = super::parse_products(csv_data).unwrap();
        let Product::Stock {
            symbol,
            name,
            outstanding_shares,
            share_price,
        } = products.pop().unwrap();
        assert_eq!(symbol, "AAPL");
        assert_eq!(name, "Apple Inc.");
        assert_eq!(outstanding_shares, 100);
        assert_eq!(share_price.height, 123.45);
        assert_eq!(
            share_price.time,
            chrono::Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap()
        );
    }
}
