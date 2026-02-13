use crate::data::market::SharePrice;
use dioxus::prelude::*;

pub mod progress;

#[component]
pub fn ProductLabel(symbol: String, name: String) -> Element {
    rsx! {
        p { class: "title is-6", "{symbol}" }
        p { class: "subtitle is-7", "{name}" }
    }
}

#[component]
pub fn SharePriceLabel(share_price: SharePrice) -> Element {
    rsx! {
        div { class: "tags has-addons",
            span { class: "tag is-dark", "{format_share_price(&share_price)}" }
            span { class: "tag", "{format_time(&share_price.time)}"}
        }
    }
}

fn format_share_price(price: &SharePrice) -> String {
    format!("${:.2}", price.height)
}

fn format_time(time: &chrono::DateTime<chrono::Utc>) -> String {
    let local_time = time.with_timezone(&chrono::Local);
    local_time
        .format("%b\u{202f}%-e,\u{202f}%Y\u{202f}@\u{202f}%-l:%M%P")
        .to_string()
}
