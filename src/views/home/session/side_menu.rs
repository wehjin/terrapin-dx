use crate::bulma::BulmaColor;
use crate::components::pill::LabelPill;
use crate::views::home::session::Tab;
use dioxus::prelude::*;

#[component]
pub fn SideMenu(user_name: String, active_tab: Signal<Tab>) -> Element {
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
        p { class: "menu-label", "Actions"}
        ul { class: "menu-list",
            TabListItem { tab: Tab::ImportPrices, active: active_tab }
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
        Tab::ImportPrices => "Import Prices",
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
