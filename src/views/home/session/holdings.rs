use crate::components::progress::ProgressIndicator;
use crate::components::ProductLabel;
use crate::data::market::Product;
use crate::data::ownership::Ownership;
use crate::data::portfolio::Lot;
use crate::server::SessionState;
use chrono::{DateTime, Utc};
use dioxus::html::completions::CompleteWithBraces::progress;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Holdings(session: ReadSignal<SessionState>) -> Element {
    let products_by_symbol = use_memo(move || {
        session()
            .products
            .clone()
            .into_iter()
            .map(|product| (product.symbol().to_string(), product))
            .collect::<HashMap<String, Product>>()
    });
    let subtitle = format!("{}", session().login_name);
    let mut holding_rows = holding_rows(session().lots, products_by_symbol(), Utc::now());
    holding_rows.sort_by(|a, b| match (a.ownership, b.ownership) {
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (_, _) => std::cmp::Ordering::Equal,
    });
    rsx! {
        div { class: "block level",
            div { class: "level-left",
                h1 { class: "level-item title", "Holdings" }
                h2 { class: "level-item subtitle",
                    div { class: "tags has-addons",
                        span { class: "tag", "User" }
                        span { class: "tag is-info", "{subtitle}" }
                    }
                }
            }
        }
        div { class: "block",
            table {
                class: "table is-striped",
                thead {
                    tr {
                        th { "Product" }
                        th { "Quantity" }
                        th { "Ownership" }
                    }
                }
                tbody {
                    { holding_rows.iter().map(|row| rsx! {
                        tr {
                            td {
                                ProductLabel{ symbol: row.symbol.clone(), name: row.name.clone()}
                            }
                            td {
                                QuantityTag{
                                    quantity: row.quantity,
                                    account: row.accounts.to_string()
                                }
                                TermIndicator{ long_term: row.long_term, short_term: row.short_term }
                            }
                            td {
                                match row.ownership.clone() {
                                    Some(ownership) => rsx!(OwnershipTags{ ownership }),
                                    None => rsx!(),
                                }
                            }
                        }
                    }) }
                }
            }
        }
    }
}

fn holding_rows(
    lots: Vec<Lot>,
    products: HashMap<String, Product>,
    now: DateTime<Utc>,
) -> Vec<HoldingRow> {
    let one_year_ago = now - chrono::Duration::days(365);
    let lots_by_product: HashMap<String, Vec<Lot>> =
        lots.into_iter()
            .fold(HashMap::<String, Vec<Lot>>::new(), |mut holdings, lot| {
                let symbol = lot.product.clone();
                holdings.entry(symbol).or_default().push(lot);
                holdings
            });
    let mut rows = lots_by_product
        .into_iter()
        .filter(|(symbol, _)| products.contains_key(symbol))
        .map(|(symbol, lots)| {
            let product = products.get(&symbol).unwrap();
            let name = product.name().to_string();
            let (quantity, long_term, short_term) =
                lots.iter()
                    .fold((0.0, 0.0, 0.0), |(quantity, long_term, short_term), lot| {
                        let new_quantity = quantity + lot.quantity;
                        if lot.time < one_year_ago {
                            (new_quantity, long_term + lot.quantity, short_term)
                        } else {
                            (new_quantity, long_term, short_term + lot.quantity)
                        }
                    });
            let ownership = match product.supply() {
                Some(value) => Some(Ownership::new(quantity, value)),
                None => None,
            };
            HoldingRow {
                symbol,
                name,
                accounts: format_accounts(&lots),
                quantity: quantity.floor() as usize,
                ownership,
                long_term,
                short_term,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    rows
}

fn format_accounts(lots: &Vec<Lot>) -> String {
    let mut account_shares = {
        let mut map = HashMap::<String, f64>::new();
        for lot in lots {
            *map.entry(lot.account.clone()).or_default() += lot.quantity;
        }
        let vec = map.into_iter().collect::<Vec<_>>();
        vec
    };

    account_shares.sort_by(|a, b| a.1.total_cmp(&b.1));
    let first = account_shares.remove(account_shares.len() - 1);
    if account_shares.is_empty() {
        first.0.clone()
    } else {
        format!("{}\u{202f}+\u{202f}{}", first.0, account_shares.len())
    }
}

#[derive(Debug, Clone)]
struct HoldingRow {
    symbol: String,
    name: String,
    accounts: String,
    quantity: usize,
    ownership: Option<Ownership>,
    long_term: f64,
    short_term: f64,
}

#[component]
fn QuantityTag(quantity: usize, account: String) -> Element {
    rsx! {
        div { class: "tags",
            span { class: "tag is-primary", "{quantity}" }
            span { class: "tag is-info", "{account}" }
        }
    }
}

#[component]
fn TermIndicator(long_term: f64, short_term: f64) -> Element {
    let total = long_term + short_term;
    rsx! {
        ProgressIndicator{
            title: "Long-term".to_string(),
            progress: long_term.round() as usize,
            total: total.ceil() as usize,
        }
    }
}

#[component]
fn OwnershipTags(ownership: Ownership) -> Element {
    let progress = (ownership.progress() * 100.0).floor() as u8;
    let reach = 100 - progress;
    let rank = format!("{}{:02}", ownership.level, progress);

    rsx! {
        div { class: "tags has-addons",
                span { class: "tag is-dark", "Level" }
                span { class: "tag is-primary", "{rank}" }
        }
        ProgressIndicator{
            title: "Progress".to_string(),
            progress: ownership.excess_shares,
            total: ownership.total_shares()
        }
    }
}
