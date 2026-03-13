use crate::data::market::SharePrice;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketPrice {
    pub symbol: String,
    pub share_price: SharePrice,
}

#[derive(Error, Debug)]
pub enum CsvParseError {
    #[error("Csv read error: {0}")]
    CsvReadError(#[from] csv::Error),

    #[error("Chrono parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Chrono tz parse error: {0}")]
    ChronoTzParseError(#[from] chrono_tz::ParseError),
}

pub fn parse_market_prices(csv_bytes: &[u8]) -> Result<Vec<MarketPrice>, CsvParseError> {
    #[derive(Debug, Deserialize)]
    struct Row {
        #[serde(rename = "Symbol")]
        symbol: String,
        #[serde(rename = "Current Price")]
        price: f64,
        #[serde(rename = "Date")]
        date: String,
        #[serde(rename = "Time")]
        time: String,
    }
    let rows = csv::Reader::from_reader(csv_bytes)
        .deserialize()
        .collect::<Result<Vec<Row>, _>>()?;
    let mut samples = Vec::new();
    for row in rows {
        let share_price = SharePrice {
            height: row.price,
            time: parse_date_time(&row.date, &row.time)?,
        };
        let sample = MarketPrice {
            symbol: row.symbol,
            share_price,
        };
        samples.push(sample);
    }
    Ok(samples)
}

fn parse_date_time(date: &str, time: &str) -> Result<DateTime<chrono::Utc>, CsvParseError> {
    let string = format!("{} {}", date, time);
    let parts = string.rsplitn(2, ' ').collect::<Vec<_>>();
    let (tz_abbrev, date_time_str) = (parts[0], parts[1]);
    let tz: Tz = match tz_abbrev {
        "EST" | "EDT" => "America/New_York",
        "CST" | "CDT" => "America/Chicago",
        "MST" | "MDT" => "America/Denver",
        "PST" | "PDT" => "America/Los_Angeles",
        "BST" => "Europe/London",
        "UTC" => "UTC",
        _ => "UTC", // Fallback guess
    }
    .parse()?;

    let native_dt = NaiveDateTime::parse_from_str(date_time_str, "%Y/%m/%d %H:%M")?;
    let utc = tz
        .from_local_datetime(&native_dt)
        .earliest()
        .expect("Invalid time for timezone")
        .with_timezone(&chrono::Utc);
    Ok(utc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_chrono_parse() {
        let date = "2026/03/13";
        let time = "14:07 EDT";
        let fixed = parse_date_time(date, time).unwrap();
        assert_eq!(fixed.year(), 2026);
    }
}
