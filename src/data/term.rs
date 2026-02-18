use crate::data::market::Product;
use crate::data::portfolio::Lot;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct TermReport {
    pub symbol: String,
    pub long_term: f64,
    pub short_term: f64,
    pub wash: f64,
    pub short_exit: Option<DateTime<Utc>>,
    pub wash_exit: Option<DateTime<Utc>>,
}

impl TermReport {
    pub fn new(symbol: &str, lots: &[Lot], now: DateTime<Utc>) -> Self {
        const WASH_DURATION: Duration = Duration::days(32);
        const SHORT_DURATION: Duration = Duration::days(365);
        let one_year_ago = now - SHORT_DURATION;
        let wash_start = now - WASH_DURATION;
        let mut report = TermReport {
            symbol: symbol.to_string(),
            long_term: 0.0,
            short_term: 0.0,
            wash: 0.0,
            short_exit: None,
            wash_exit: None,
        };
        for lot in lots {
            if lot.time < one_year_ago {
                report.long_term += lot.quantity;
            } else if lot.time < wash_start {
                report.short_term += lot.quantity;
                let this_exit = lot.time + SHORT_DURATION;
                report.short_exit = match report.short_exit {
                    None => Some(this_exit),
                    Some(previous) => Some(previous.max(this_exit)),
                };
            } else {
                report.wash += lot.quantity;
                let this_exit = lot.time + WASH_DURATION;
                report.wash_exit = match report.wash_exit {
                    None => Some(this_exit),
                    Some(previous) => Some(previous.max(this_exit)),
                };
            }
        }
        report
    }
}

pub fn term_reports(
    lots_by_product: &HashMap<String, Vec<Lot>>,
    products: &HashMap<String, Product>,
    now: DateTime<Utc>,
) -> HashMap<String, TermReport> {
    let mut reports: HashMap<String, TermReport> = HashMap::new();
    for (symbol, lots) in lots_by_product {
        if products.contains_key(symbol) {
            let report = TermReport::new(symbol, lots, now);
            reports.insert(symbol.clone(), report);
        }
    }
    reports
}
