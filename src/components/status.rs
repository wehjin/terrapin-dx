use dioxus::prelude::*;

#[component]
pub fn StatusMessage(message: String) -> Element {
    rsx! {
        article { class: "message",
            div { class: "message-header", "Status" }
            div { class: "message-body", "{message}" }
        }
    }
}
