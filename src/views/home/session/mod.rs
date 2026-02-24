use crate::api::session::SessionState;
use dioxus::prelude::*;

mod holdings;
use holdings::Holdings;
mod products;
use products::Products;

mod lots;
use crate::components::pill::LabelPill;
use lots::Lots;
use crate::bulma::BulmaColor;

#[component]
pub fn Session(session: ReadSignal<SessionState>) -> Element {
    let tab = use_signal(|| Tab::Holdings);
    let user_name = mask_middle_chars(&session().login_name);
    rsx! {
        div { class: "columns p-2",
            aside { class: "column is-narrow menu",
                p { class: "menu-list",
                    LabelPill { label: "User", value: user_name, color: BulmaColor::Light }
                }
                p { class: "menu-label", "Treasury" }
                ul { class: "menu-list",
                    TabListItem { tab: Tab::Holdings, active: tab }
                    TabListItem { tab: Tab::Lots, active: tab }
                }
                p { class: "menu-label", "Market" }
                ul { class: "menu-list",
                    TabListItem { tab: Tab::Products, active: tab }
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

fn mask_middle_chars(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    if chars.len() > 2 {
        for i in 1..chars.len() - 1 {
            chars[i] = '-';
        }
    }

    chars.into_iter().collect()
}

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
