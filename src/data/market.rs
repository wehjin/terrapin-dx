use crate::data::{FormatError, ParseError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharePrice {
    #[serde(rename = "share_price")]
    pub height: f64,
    #[serde(rename = "share_price_as_of")]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Product {
    Stock {
        symbol: String,
        name: String,
        outstanding_shares: usize,
        #[serde(flatten)]
        share_price: SharePrice,
    },
    Etf {
        symbol: String,
        name: String,
        #[serde(flatten)]
        share_price: SharePrice,
    },
    Coin {
        symbol: String,
        name: String,
        #[serde(rename = "outstanding_shares")]
        total_supply: usize,
        #[serde(flatten)]
        share_price: SharePrice,
    },
    Note {
        symbol: String,
        name: String,
        #[serde(flatten)]
        share_price: SharePrice,
    },
}

impl Product {
    pub fn symbol(&self) -> &str {
        match self {
            Product::Stock { symbol, .. } => symbol,
            Product::Etf { symbol, .. } => symbol,
            Product::Coin { symbol, .. } => symbol,
            Product::Note { symbol, .. } => symbol,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Product::Stock { name, .. } => name,
            Product::Etf { name, .. } => name,
            Product::Coin { name, .. } => name,
            Product::Note { name, .. } => name,
        }
    }
    pub fn supply(&self) -> Option<usize> {
        match self {
            Product::Stock {
                outstanding_shares, ..
            } => Some(*outstanding_shares),
            Product::Etf { .. } => None,
            Product::Coin { total_supply, .. } => Some(*total_supply),
            Product::Note { .. } => None,
        }
    }

    pub fn share_price(&self) -> &SharePrice {
        match self {
            Product::Stock { share_price, .. } => share_price,
            Product::Etf { share_price, .. } => share_price,
            Product::Coin { share_price, .. } => share_price,
            Product::Note { share_price, .. } => share_price,
        }
    }
    pub fn set_share_price(&mut self, share_price: SharePrice) {
        match self {
            Product::Stock {
                share_price: ref mut sp,
                ..
            } => *sp = share_price,
            Product::Etf {
                share_price: ref mut sp,
                ..
            } => *sp = share_price,
            Product::Coin {
                share_price: ref mut sp,
                ..
            } => *sp = share_price,
            Product::Note {
                share_price: ref mut sp,
                ..
            } => *sp = share_price,
        }
    }
}

// Flattening into a proxy works around an issue with deserializing enums with interior flattened fields.
#[derive(Debug, Serialize, Deserialize)]
struct ProductProxy {
    #[serde(flatten)]
    product: Product,
}

pub fn parse_products(csv_data: &[u8]) -> Result<Vec<Product>, ParseError> {
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

pub fn format_products(products: Vec<Product>) -> Result<String, FormatError> {
    let rows = products
        .into_iter()
        .map(ProductRow::from)
        .collect::<Vec<_>>();
    let mut writer = csv::Writer::from_writer(vec![]);
    for row in rows.iter() {
        writer.serialize(row)?;
    }
    let data = writer.into_inner()?;
    let string = String::from_utf8(data)?;
    Ok(string)
}

#[derive(Serialize)]
struct ProductRow {
    #[serde(rename = "type")]
    type_: &'static str,
    symbol: String,
    name: String,
    outstanding_shares: usize,
    share_price: f64,
    share_price_as_of: chrono::DateTime<chrono::Utc>,
}

impl From<Product> for ProductRow {
    fn from(value: Product) -> Self {
        match value {
            Product::Stock {
                symbol,
                name,
                outstanding_shares,
                share_price,
            } => ProductRow {
                type_: "stock",
                symbol,
                name,
                outstanding_shares,
                share_price: share_price.height,
                share_price_as_of: share_price.time,
            },
            Product::Etf {
                symbol,
                name,
                share_price,
            } => ProductRow {
                type_: "etf",
                symbol,
                name,
                outstanding_shares: 0,
                share_price: share_price.height,
                share_price_as_of: share_price.time,
            },
            Product::Coin {
                symbol,
                name,
                total_supply,
                share_price,
            } => ProductRow {
                type_: "coin",
                symbol,
                name,
                outstanding_shares: total_supply,
                share_price: share_price.height,
                share_price_as_of: share_price.time,
            },
            Product::Note {
                symbol,
                name,
                share_price,
            } => ProductRow {
                type_: "note",
                symbol,
                name,
                outstanding_shares: 0,
                share_price: share_price.height,
                share_price_as_of: share_price.time,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::market::{Product, SharePrice};
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_format_products() {
        let products = vec![Product::Stock {
            symbol: "RKLB".into(),
            name: "Rocket Lab, Inc.".into(),
            outstanding_shares: 1000,
            share_price: SharePrice {
                height: 80.80,
                time: Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap(),
            },
        }];
        let string = super::format_products(products).unwrap();
        assert_eq!(
			string,
			"type,symbol,name,outstanding_shares,share_price,share_price_as_of\nstock,RKLB,\"Rocket Lab, Inc.\",1000,80.8,2021-01-01T00:00:00Z\n"
		);
    }

    #[test]
    fn test_parse_products() {
        let csv_data = r#"
        type,symbol,name,outstanding_shares,share_price,share_price_as_of
        stock,AAPL,Apple Inc.,100,123.45,2021-01-01T00:00:00Z
        etf,CMF,iShares California Muni Bond ETF,,57.85,2026-01-30T16:26:31Z
        coin,ETH,Ethereum,120690000,2722.99,2026-01-30T04:51:00Z
        note,USD,US Dollar Credits,,1.0,1971-08-16T01:00:00Z
        "#
        .trim()
        .as_bytes();
        let vec = super::parse_products(csv_data).unwrap();
        let array = vec.as_array().unwrap();
        assert_eq!(
            array,
            &[
                Product::Stock {
                    symbol: "AAPL".to_string(),
                    name: "Apple Inc.".to_string(),
                    outstanding_shares: 100,
                    share_price: SharePrice {
                        height: 123.45,
                        time: chrono::Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap()
                    },
                },
                Product::Etf {
                    symbol: "CMF".to_string(),
                    name: "iShares California Muni Bond ETF".to_string(),
                    share_price: SharePrice {
                        height: 57.85,
                        time: chrono::Utc
                            .with_ymd_and_hms(2026, 1, 30, 16, 26, 31)
                            .unwrap()
                    },
                },
                Product::Coin {
                    symbol: "ETH".to_string(),
                    name: "Ethereum".to_string(),
                    total_supply: 120690000,
                    share_price: SharePrice {
                        height: 2722.99,
                        time: chrono::Utc.with_ymd_and_hms(2026, 1, 30, 4, 51, 0).unwrap()
                    },
                },
                Product::Note {
                    symbol: "USD".to_string(),
                    name: "US Dollar Credits".to_string(),
                    share_price: SharePrice {
                        height: 1.0,
                        time: chrono::Utc.with_ymd_and_hms(1971, 8, 16, 1, 0, 0).unwrap()
                    },
                }
            ]
        );
    }
}
