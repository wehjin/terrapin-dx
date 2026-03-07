use crate::data::market::{Product, SharePrice};
use crate::data::portfolio::Lot;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};

pub struct NetWorthReport {
    pub pre_tax: f64,
    pub as_of: DateTime<Utc>,
    pub unpriced_products: HashSet<String>,
}

impl NetWorthReport {
    pub fn new(lots: &Vec<Lot>, products: &Vec<Product>) -> Self {
        let price_map = to_price_map(products);
        let mut pre_tax = 0.0;
        let mut as_of = DateTime::<Utc>::MIN_UTC;
        let mut unpriced_products: HashSet<String> = HashSet::new();
        for lot in lots {
            let product_id = &lot.product;
            let share_price = price_map.get(product_id);
            if let Some(share_price) = share_price {
                let dollars = lot.quantity * share_price.height;
                pre_tax += dollars;
                as_of = as_of.max(share_price.time.clone());
            } else {
                unpriced_products.insert(product_id.to_string());
            }
        }
        Self {
            pre_tax,
            as_of,
            unpriced_products,
        }
    }
}

fn to_price_map(products: &Vec<Product>) -> HashMap<String, SharePrice> {
    let mut map: HashMap<String, SharePrice> = HashMap::new();
    for p in products {
        map.insert(p.symbol().to_string(), p.share_price().clone());
    }
    map
}
