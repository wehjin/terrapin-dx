use crate::api::{query_lots, query_products};
use crate::data::net_worth::NetWorthReport;
use dioxus::prelude::*;

#[component]
pub fn NetWorthPage() -> Element {
    let products = use_loader(|| async move { query_products().await })?;
    let lots = use_loader(|| async move {
        query_lots()
            .await
            .map(|items| items.into_iter().map(|item| item.0).collect::<Vec<_>>())
    })?;
    let report = NetWorthReport::new(&lots(), &products());
    let pre_tax = format_dollars(report.pre_tax);
    let as_of = format_date(report.as_of);
    let unpriced_products = report.unpriced_products.len();
    rsx! {
        h1 { class: "title", "Net Worth" }
        h5 { class: "title is-5", "Summary" }
        nav { class: "level is-mobile",
            LevelDetailItem { label: "Pre tax".to_string(), content: pre_tax}
            LevelDetailItem { label: "As of".to_string(), content: as_of }
            LevelDetailItem { label: "Unpriced".to_string(), content: unpriced_products.to_string() }
        }
    }
}

#[component]
fn LevelDetailItem(label: String, content: String) -> Element {
    rsx! {
        div { class: "level-item has-text-centered",
            div {
                p { class: "heading", {label} }
                p { class: "title", {content} }
            }
        }
    }
}

pub fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%-m/%-d/%y").to_string()
}

pub fn format_dollars(amount: f64) -> String {
    let digits = amount.round().to_string();
    let unit = match digits.len() {
        0 => Unit::Zero,
        1..=3 => Unit::Ones,
        4..=6 => Unit::Thousands,
        7..=9 => Unit::Millions,
        10..=12 => Unit::Billions,
        13..=15 => Unit::Trillions,
        _ => Unit::ALot,
    };
    unit.format_digits(&digits, "$")
}

enum Unit {
    Zero,
    Ones,
    Thousands,
    Millions,
    Billions,
    Trillions,
    ALot,
}
impl Unit {
    pub fn format_digits(&self, digits: &str, prefix: &str) -> String {
        let text = match self {
            Unit::Zero => "0".to_string(),
            Unit::Ones => digits.to_string(),
            Unit::Thousands | Unit::Millions | Unit::Billions | Unit::Trillions => {
                let head_len = digits.len() - self.floor_len();
                let tail_len = 3 - head_len;
                let head = digits[0..head_len].to_string();
                if tail_len > 0 {
                    let tail = digits[head_len..(head_len + tail_len)].to_string();
                    format!("{}.{}", head, tail)
                } else {
                    head
                }
            }
            Unit::ALot => "∞".to_string(),
        };
        format!("{}{}\u{202f}{}", prefix, text, self.suffix())
    }
    fn suffix(&self) -> &'static str {
        match self {
            Unit::Zero => "",
            Unit::Ones => "",
            Unit::Thousands => "K",
            Unit::Millions => "M",
            Unit::Billions => "B",
            Unit::Trillions => "T",
            Unit::ALot => panic!("ALot has no suffix"),
        }
    }
    fn floor_len(&self) -> usize {
        match self {
            Unit::Zero => panic!("Zero has no floor length"),
            Unit::Ones => 0,
            Unit::Thousands => 3,
            Unit::Millions => 6,
            Unit::Billions => 9,
            Unit::Trillions => 12,
            Unit::ALot => 15,
        }
    }
}
