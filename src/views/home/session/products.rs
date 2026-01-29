use crate::components::{ProductLabel, SharePriceLabel};
use crate::server::SessionState;
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
                        th { "Shares" }
                        th { "Price ⁄ Share" }
                    }
                }
                tbody {
                    for product in &products {
                        tr {
                            td { ProductLabel{ symbol: product.symbol(), name: product.name() } }
                            td { "{format_outstanding_shares(product.outstanding_shares())}" }
                            td { SharePriceLabel{ share_price: product.share_price().clone()} }
                        }
                    }
                }
            }
        }
    }
}

fn format_outstanding_shares(shares: Option<usize>) -> String {
    match shares {
        None => String::new(),
        Some(n) => n
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("\u{202f}"),
    }
}
