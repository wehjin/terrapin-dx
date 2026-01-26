use dioxus::prelude::*;
#[component]
pub fn Invalid(login_name: String, message: String, on_retry: EventHandler<()>) -> Element {
    rsx! {
        section {
            class: "section",
            div { class: "container",
                section { class: "section",
                    h1 { class: "title", "Invalid" }
                    h2 { class: "subtitle", "{login_name}" }
                    div { class: "box", "{message}" }
                    button { class: "button is-primary", onclick: move |_| on_retry.call(()), "Try again" }
                }
            }
        }
    }
}
