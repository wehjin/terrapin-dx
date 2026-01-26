use crate::server::SessionState;
use dioxus::prelude::*;

#[component]
pub fn Session(session: SessionState) -> Element {
    let content = format!("{:?}", session);
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "title", "Home" }
                h2 { class: "subtitle", "{session.login_name}" }
                p {
                    {content}
                }
            }
        }
    }
}
