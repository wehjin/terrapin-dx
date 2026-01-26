use crate::server::{load_session, SessionState};
use dioxus::prelude::*;
mod login;
use login::Login;
mod invalid;
use invalid::Invalid;
mod session;
use session::Session;

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
                    match load_session(login_name.clone()).await {
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
            Invalid {
                login_name,
                message,
                on_retry: move |_| app_state.set(AppState::Login)
            }
        },
        AppState::Active { session } => rsx! {
            Session {
                session
            }
        },
    }
}
