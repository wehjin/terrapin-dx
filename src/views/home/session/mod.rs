use crate::api::session::SessionState;
use dioxus::prelude::*;

mod holdings;
use holdings::Holdings;
mod products;
use products::Products;

mod net_worth;
use net_worth::NetWorthPage;

mod lots;
use lots::Lots;

use crate::bulma::BulmaColor;
use crate::components::pill::LabelPill;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tab {
    Holdings,
    Products,
    Lots,
    NetWorth,
}

#[component]
pub fn Session(session: ReadSignal<SessionState>) -> Element {
    let tab = use_signal(|| Tab::Holdings);
    let user_name = session().login_name.to_string();
    rsx! {
            div { class: "columns p-2",
                aside { class: "column is-narrow menu",
                    SideMenu { user_name: user_name.clone(), active_tab: tab.clone() }
                }
                main { class: "column p-4",
                    match tab() {
                        Tab::Holdings => rsx! (Holdings { session: session() }),
                        Tab::Products => rsx! (Products { session: session() }),
                        Tab::Lots => rsx!(Lots { session: session() }),
                        Tab::NetWorth => rsx!(NetWorthPage { session: session() }),
                    }
            }
        }
    }
}

#[component]
fn SideMenu(user_name: String, active_tab: Signal<Tab>) -> Element {
    rsx! {
        p { class: "menu-list",
            LabelPill { label: "User", value: user_name, color: BulmaColor::Light }
        }
        p { class: "menu-label", "Views" }
        ul { class: "menu-list",
            TabListItem { tab: Tab::Holdings, active: active_tab }
            TabListItem { tab: Tab::NetWorth, active: active_tab }
        }
        p { class: "menu-label", "Data"}
        ul { class: "menu-list",
            TabListItem { tab: Tab::Lots, active: active_tab }
            TabListItem { tab: Tab::Products, active: active_tab }
        }
    }
}

#[component]
fn TabListItem(tab: Tab, active: Signal<Tab>) -> Element {
    let label = match tab {
        Tab::Holdings => "Holdings",
        Tab::Products => "Products",
        Tab::Lots => "Lots",
        Tab::NetWorth => "Net Worth",
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
