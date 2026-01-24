use crate::server::{load_session, SessionState};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum AppState {
    Login,
    Invalid { login_name: String },
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
                        Err(_) => app_state.set(AppState::Invalid{ login_name }),
                    };
                }
            }
        },
        AppState::Invalid { login_name } => rsx! {
            section { class: "section",
                div { class: "container",
                    h1 { class: "title", "Invalid login: {login_name}" }
                    button { class: "button is-primary", onclick: move |_| app_state.set(AppState::Login), "Try again" }
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
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "title", "Home" }
                h2 { class: "subtitle", "{session.login_name}" }
            }
        }
    }
}

#[component]
fn Login(on_login: EventHandler<String>) -> Element {
    let mut submit_name = use_signal(|| None::<String>);
    rsx! {
            section { class: "section",
            div { class: "container",
                h1 { class: "title", "Login" }
                div { class: "field has-addons",
                    div { class: "control",
                        input { class: "input",
                            type: "text",
                            name: "login-name",
                            placeholder: "Enter user name",
                            oninput: move | evt | {
                                let value = evt.value().to_string();
                                if value.len() > 0 && value.chars().all( | c | c.is_alphanumeric()) {
                                    submit_name.set(Some(value));
                                } else {
                                    submit_name.set(None);
                                }
                            }
                        }
                    }
                    div { class: "control",
                        button { class: "button is-primary", type: "submit", disabled: submit_name().is_none(),
                            onclick: move |_| on_login.call(submit_name().unwrap()),
                            "Login"
                        }
                    }
                }
            }
        }
    }
}
