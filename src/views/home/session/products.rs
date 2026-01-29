use crate::server::SessionState;
use dioxus::prelude::*;

#[component]
pub fn Products(session: ReadSignal<SessionState>) -> Element {
    rsx! {
        div { class: "block level",
            div { class: "level-left",
                h1 { class: "level-item title", "Products" }
            }
        }
    }
}
