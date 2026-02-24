use crate::api::session::fetch_session;
use dioxus::prelude::*;
mod invalid;
mod session;
use crate::views::login::Login;
use session::Session;

#[component]
pub fn Home() -> Element {
    let mut active_session = use_loader(move || async move { fetch_session().await })?;
    match active_session() {
        Some(session) => rsx! {
            Session { session }
        },
        None => rsx! {
            div { class: "section",
                Login { onsuccess: move |_| {
                    active_session.restart();
                }}
            }
        },
    }
}
