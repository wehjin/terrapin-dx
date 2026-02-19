use crate::server::SessionState;
use dioxus::prelude::*;

mod holdings;
use holdings::Holdings;
mod products;
use products::Products;

mod lots;
use lots::Lots;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tab {
    Holdings,
    Products,
    Lots,
}

#[component]
fn TabListItem(tab: Tab, active: Signal<Tab>) -> Element {
    let label = match tab {
        Tab::Holdings => "Holdings",
        Tab::Products => "Products",
        Tab::Lots => "Lots",
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
        div { class: "columns p-2",
            aside { class: "column is-narrow menu",
                p { class: "menu-label", "General" }
                ul { class: "menu-list",
                    TabListItem { tab: Tab::Holdings, active: tab }
                    TabListItem { tab: Tab::Products, active: tab }
                    TabListItem { tab: Tab::Lots, active: tab }
                }
            }
            main { class: "column p-4",
                match tab() {
                    Tab::Holdings => rsx! (Holdings { session: session() }),
                    Tab::Products => rsx! (Products { session: session() }),
                    Tab::Lots => rsx!(Lots { session: session() })}
                }

        }
    }
}
