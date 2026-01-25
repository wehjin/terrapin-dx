use crate::server::{load_session, SessionState};
use crate::views::login::Login;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum AppState {
    Login,
    Invalid { login_name: String, message: String },
    Active { session: SessionState },
}

#[component]
pub fn Home() -> Element {
    let mut app_state = use_signal(|| AppState::Login);
    match app_state() {
        AppState::Login => rsx! {
            Login {
                on_login: move |login_name: String| async move {
                    let result = load_session(login_name.clone()).await;
                     match result {
                        Ok(session) => app_state.set(AppState::Active { session }),
                        Err(e) => app_state.set(AppState::Invalid { login_name, message: e.to_string() }),
                    };
                }
            }
        },
        AppState::Invalid {
            login_name,
            message,
        } => rsx! {
            section { class: "section",
                div { class: "container",
                    section { class: "section",
                        h1 { class: "title", "Invalid" }
                        h2 { class: "subtitle", "{login_name}" }
                        div { class: "box", "{message}" }
                        button { class: "button is-primary", onclick: move |_| app_state.set(AppState::Login), "Try again" }
                    }
                }
            }
        },
        AppState::Active { session } => rsx! {
            Session { session }
        },
    }
}

#[component]
fn Session(session: SessionState) -> Element {
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
