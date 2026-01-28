use crate::data::market::Product;
use crate::data::ownership::Ownership;
use crate::data::portfolio::Lot;
use crate::server::SessionState;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

#[component]
pub fn Session(session: ReadSignal<SessionState>) -> Element {
    let products_by_symbol = use_memo(move || {
        session()
            .products
            .clone()
            .into_iter()
            .map(|product| (product.symbol().to_string(), product))
            .collect::<HashMap<String, Product>>()
    });
    let subtitle = format!("{}", session().login_name);
    let holding_rows = holding_rows(session().lots, products_by_symbol());

    rsx! {
        section { class: "section",
            div { class: "container",
                div { class: "block",
                    div { class: "level",
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
                }
                div { class: "block",
                    table {
                        class: "table is-striped",
                        thead {
                            tr {
                                th { "Product" }
                                th { "Accounts" }
                                th { "Quantity" }
                                th { "Ownership" }
                            }
                        }
                        tbody {
                            { holding_rows.iter().map(|row| rsx! {
                                tr {
                                    td {
                                        p { class: "title is-6", "{row.symbol}" }
                                        p { class: "subtitle is-7", "{row.name}" }
                                    }
                                    td { "{row.accounts}" }
                                    td { "{row.quantity}" }
                                    td { "{row.ownership}" }
                                }
                            }) }
                        }
                    }
                }
            }
        }
    }
}

fn holding_rows(lots: Vec<Lot>, products: HashMap<String, Product>) -> Vec<HoldingRow> {
    let lots_by_product: HashMap<String, Vec<Lot>> =
        lots.into_iter()
            .fold(HashMap::<String, Vec<Lot>>::new(), |mut holdings, lot| {
                let symbol = lot.product.clone();
                holdings.entry(symbol).or_default().push(lot);
                holdings
            });
    let rows = lots_by_product
        .into_iter()
        .filter(|(symbol, _)| products.contains_key(symbol))
        .map(|(symbol, lots)| {
            let product = products.get(&symbol).unwrap();
            let name = product.name().to_string();
            let Product::Stock {
                outstanding_shares, ..
            } = product;
            let quantity = lots.iter().fold(0.0, |acc, lot| acc + lot.quantity);
            let ownership = Ownership::new(quantity, *outstanding_shares);
            HoldingRow {
                symbol,
                name,
                accounts: format_accounts(&lots),
                quantity: quantity.to_string(),
                ownership: ownership.to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows
}

fn format_accounts(lots: &Vec<Lot>) -> String {
    let mut accounts = lots
        .iter()
        .map(|lot| lot.account.to_string())
        .collect::<HashSet<String>>();
    let mut sorted = accounts.drain().collect::<Vec<_>>();
    sorted.sort();
    sorted.join(", ")
}

#[derive(Debug, Clone)]
struct HoldingRow {
    symbol: String,
    name: String,
    accounts: String,
    quantity: String,
    ownership: String,
}
