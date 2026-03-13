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

mod side_menu;
use side_menu::SideMenu;

mod import_prices;
use import_prices::ImportPrices;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tab {
    Holdings,
    Products,
    Lots,
    NetWorth,
    ImportPrices,
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
                        Tab::ImportPrices => rsx!(ImportPrices {}),
                    }
            }
        }
    }
}
