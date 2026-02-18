use crate::components::pill::{BulmaColor, LabelPill};
use crate::components::progress::ProgressIndicator;
use crate::components::ProductLabel;
use crate::data::market::Product;
use crate::data::ownership::Ownership;
use crate::data::portfolio::Lot;
use crate::server::SessionState;
use chrono::{DateTime, Utc};
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
                    LabelPill { label: "User", value: subtitle, color: BulmaColor::Link }
                }
            }
        }
        div { class: "block",
            table {
                class: "table is-striped",
                thead {
                    tr {
                        th { "Product" }
                        th { "Tax" }
                        th { "Ownership" }
                    }
                }
                tbody {
                    { holding_rows.iter().map(|row| rsx! {
                        tr {
                            // Product
                            td {
                                ProductLabel{ symbol: row.symbol.clone(), name: row.name.clone()}
                                QuantityTag{
                                    quantity: row.quantity,
                                    account: row.accounts.to_string()
                                }
                            }
                            // Term
                            td {
                                TermIndicator{ long_term: row.long_term, short_term: row.short_term, wash: row.wash}
                            }
                            // Ownership
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
    let wash_start = now - chrono::Duration::days(32);
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
            let (quantity, long_term, short_term, wash) = lots.iter().fold(
                (0.0, 0.0, 0.0, 0.0),
                |(quantity, long_term, short_term, wash), lot| {
                    let new_quantity = quantity + lot.quantity;
                    if lot.time < one_year_ago {
                        (new_quantity, long_term + lot.quantity, short_term, wash)
                    } else if lot.time < wash_start {
                        (new_quantity, long_term, short_term + lot.quantity, wash)
                    } else {
                        (new_quantity, long_term, short_term, wash + lot.quantity)
                    }
                },
            );
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
                wash,
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
    wash: f64,
}

#[component]
fn QuantityTag(quantity: usize, account: String) -> Element {
    rsx! {
        LabelPill { label: "Shares", value: quantity.to_string(), color: BulmaColor::Primary }
        LabelPill { label: "Location", value: account, color: BulmaColor::Link }
    }
}

#[component]
fn TermIndicator(long_term: f64, short_term: f64, wash: f64) -> Element {
    let long_term = long_term.ceil() as usize;
    let short_term = short_term.ceil() as usize;
    let wash = wash.ceil() as usize;
    rsx! {
        if long_term > 0 {
            LabelPill { label: "Long", value: long_term, color: BulmaColor::Success }
        }
        if short_term > 0 {
            LabelPill { label: "Short", value: short_term, color: BulmaColor::Warning }
        }
        if wash > 0 {
            LabelPill { label: "Wash", value: wash, color: BulmaColor::Danger }
        }
    }
}

#[component]
fn OwnershipTags(ownership: Ownership) -> Element {
    let color = BulmaColor::Primary;
    rsx! {
        div { class: "tags has-addons",
                span { class: "tag is-dark", "Level" }
                span { class: "tag", class: "{color.class()}", "{ownership.level}" }
        }
        ProgressIndicator{
            title: "Next level:".to_string(),
            progress: ownership.excess_shares,
            total: ownership.total_shares()
        }
    }
}
