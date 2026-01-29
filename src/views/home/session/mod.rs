use crate::server::SessionState;
use dioxus::prelude::*;

mod products;
use products::Products;
mod holdings;
use holdings::Holdings;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tab {
    Holdings,
    Products,
}

#[component]
fn TabListItem(tab: Tab, active: Signal<Tab>) -> Element {
    let label = match tab {
        Tab::Holdings => "Holdings",
        Tab::Products => "Products",
    };
    rsx! {
        li {
            a {
                class: if tab == active() { "is-active" },
                onclick: move |_| active.set(tab),
                "{label}"
            }
        }
    }
}

#[component]
pub fn Session(session: ReadSignal<SessionState>) -> Element {
    let tab = use_signal(|| Tab::Holdings);
    rsx! {
        div { class: "columns pl-4",
            aside { class: "column is-2 menu",
                p { class: "menu-label", "General" }
                ul { class: "menu-list",
                    TabListItem { tab: Tab::Holdings, active: tab }
                    TabListItem { tab: Tab::Products, active: tab }
                }
            }
            main { class: "column",
                match tab() {
                    Tab::Holdings => rsx! {
                        Holdings { session: session() }
                    },
                    Tab::Products => rsx! {
                        Products { session: session() }
                    },
                }
            }
        }
    }
}
