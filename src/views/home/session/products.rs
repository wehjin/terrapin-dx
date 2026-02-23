use crate::components::{ProductLabel, SharePriceLabel};
use crate::api::session::SessionState;
use dioxus::prelude::*;

#[component]
pub fn Products(session: ReadSignal<SessionState>) -> Element {
    let mut products = session().products.clone();
    products.sort_by(|a, b| a.symbol().cmp(&b.symbol()));
    rsx! {
        div { class: "block level",
            div { class: "level-left",
                h1 { class: "level-item title", "Products" }
            }
        }
        div { class: "block",
            table { class: "table is-striped",
                thead {
                    tr {
                        th { "Symbol" }
                        th { "Supply" }
                        th { "Price ⁄ Unit" }
                    }
                }
                tbody {
                    for product in &products {
                        tr {
                            td { ProductLabel{ symbol: product.symbol(), name: product.name() } }
                            td { SupplyLabel{ supply: product.supply() } }
                            td { SharePriceLabel{ share_price: product.share_price().clone()} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SupplyLabel(supply: Option<usize>) -> Element {
    let label = match supply {
        None => "N/A".to_string(),
        Some(n) => n
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()?
            .join("\u{202f}"),
    };
    rsx! { "{label}" }
}
