use dioxus::prelude::*;

#[component]
pub fn ErrorMessage(error_type: String, message: String, mut onretry: EventHandler<()>) -> Element {
    rsx! {
        article { class: "message is-danger",
            div { class: "message-header", "{error_type}" }
            div { class: "message-body", "{message}" }
        }
        button { class: "button is-primary", autofocus: true, onclick: move |_| onretry.call(()), "Try Again" }
    }
}