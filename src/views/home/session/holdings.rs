use crate::components::pill::LabelPill;
use crate::components::progress::ProgressIndicator;
use crate::components::ProductLabel;
use crate::data::market::Product;
use crate::data::ownership::Ownership;
use crate::data::portfolio::Lot;
use crate::data::term::{term_reports, TermReport};
use crate::api::session::SessionState;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use std::collections::HashMap;
use crate::bulma::BulmaColor;

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
    let mut holding_rows = holding_rows(session().lots, products_by_symbol(), Utc::now());
    holding_rows.sort_by(|a, b| match (a.ownership, b.ownership) {
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (_, _) => std::cmp::Ordering::Equal,
    });
    rsx! {
        h1 { class: "title", "Holdings" }
        div { class: "table-container",
            table {
                class: "table",
                thead {
                    tr {
                        th { "Asset" }
                        th { "Level" }
                        th { "Term" }
                    }
                }
                tbody {
                    { holding_rows.iter().map(|row| rsx! {
                        tr {
                            // Product
                            td {
                                ProductLabel{ symbol: row.symbol.clone(), name: row.name.clone()}
                                LabelPill { label: row.accounts.clone(), value: row.quantity.to_string(), color: BulmaColor::Light }
                            }
                            // Level
                            td {
                                match row.ownership.clone() {
                                    Some(ownership) => rsx!(OwnershipTags{ ownership }),
                                    None => rsx!(),
                                }
                            }
                            // Term
                            td {
                                TermIndicator{ term_report: row.term_report.clone() }
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
    let lots_by_product: HashMap<String, Vec<Lot>> =
        lots.into_iter()
            .fold(HashMap::<String, Vec<Lot>>::new(), |mut holdings, lot| {
                let symbol = lot.product.clone();
                holdings.entry(symbol).or_default().push(lot);
                holdings
            });

    let term_reports = term_reports(&lots_by_product, &products, now);
    let mut rows = lots_by_product
        .into_iter()
        .filter(|(symbol, _)| products.contains_key(symbol))
        .map(|(symbol, lots)| {
            let product = products.get(&symbol).unwrap();
            let name = product.name().to_string();
            let quantity = lots
                .iter()
                .fold(0.0, |quantity, lot| quantity + lot.quantity);
            let ownership = match product.supply() {
                Some(value) => Some(Ownership::new(quantity, value)),
                None => None,
            };
            let term_report = term_reports.get(&symbol).unwrap().clone();
            HoldingRow {
                symbol,
                name,
                accounts: format_accounts(&lots),
                quantity: quantity.floor() as usize,
                ownership,
                term_report,
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
    term_report: TermReport,
}

#[component]
fn TermIndicator(term_report: TermReport) -> Element {
    let long_term = term_report.long_term.ceil() as usize;
    let long_exit = Some("\u{00a0}\u{00a0}∞\u{00a0}\u{00a0}".to_string());
    let short_term = term_report.short_term.ceil() as usize;
    let short_exit = term_report
        .short_exit
        .map(|exit| exit.format("%b %-d").to_string());
    let wash = term_report.wash.ceil() as usize;
    let wash_exit = term_report
        .wash_exit
        .map(|exit| exit.format("%b %-d").to_string());
    rsx! {
        if wash > 0 {
            LabelPill { label: "Wash", value: wash, color: BulmaColor::Danger, tail: wash_exit }
        }
        if short_term > 0 {
            LabelPill { label: "Short", value: short_term, color: BulmaColor::Warning, tail: short_exit }
        }
        if long_term > 0 {
            LabelPill { label: "Long", value: long_term, color: BulmaColor::Success, tail: long_exit }
        }
    }
}

#[component]
fn OwnershipTags(ownership: Ownership) -> Element {
    rsx! {
        LabelPill { label: "Level", value: ownership.level, color: BulmaColor::Primary }
        ProgressIndicator{
            title: "Level up".to_string(),
            progress: ownership.excess_shares,
            total: ownership.total_shares()
        }
    }
}
