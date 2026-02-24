use crate::api;
use crate::components::error::ErrorMessage;
use crate::components::status::StatusMessage;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Ready,
    RequestingChallenge,
    Success,
    ServerError(String),
    BrowserError(String),
}

#[component]
pub fn TestLogin() -> Element {
    rsx! {
        div { class: "section",
            Login { onsuccess: move |_| {} }
        }
    }
}

#[component]
pub fn Login(onsuccess: EventHandler<()>) -> Element {
    let mut user_loader = use_loader(move || async move { api::active_user().await })?;
    match user_loader() {
        Some(user) => {
            onsuccess.call(());
            rsx!(StatusMessage {
                message: "Logged in as {user.username}"
            })
        }
        None => {
            rsx!(Auth {
                onsuccess: move |_| user_loader.restart()
            })
        }
    }
}

#[component]
fn Auth(onsuccess: EventHandler<()>) -> Element {
    let mut status = use_signal(|| Status::Ready);
    let mut username = use_signal(|| "".to_string());
    rsx! {
        h1 { class: "title", "Login" }
        match status() {
            Status::Ready => rsx!(
                div { class: "field",
                    label { class: "label", "Username" }
                    input { class: "input",
                        autocomplete: "username",
                        autofocus: true,
                        oninput: move |e| { username.set(e.value()) },
                        onkeydown: move |e| async move {
                            if e.key() == Key::Enter {
                                authenticate_passkey(username(), status.clone()).await;
                            }
                        }
                    }
                }
                button { class: "button",
                    onclick: move |_| async move {
                        authenticate_passkey(username(), status.clone()).await;
                    },
                    "Login"
                }
            ),
            Status::RequestingChallenge => rsx!(
                StatusMessage { message: "Fetching challenge" }
            ),
            Status::Success => {
                onsuccess.call(());
                rsx!(StatusMessage { message: "Success!"})
            },
            Status::ServerError(e) => rsx!(
                ErrorMessage { error_type: "Server Error", message: e, onretry: move |_| status.set(Status::Ready) }
            ),
            Status::BrowserError(e) => rsx!(
                ErrorMessage { error_type: "Browser Error", message: e, onretry: move |_| status.set(Status::Ready) }
            ),
        }
    }
}

async fn authenticate_passkey(name: String, mut status: Signal<Status>) {
    #[cfg(not(feature = "web"))]
    {
        status.set(Status::ServerError(
            "This platform does not support WebAuthn.".into(),
        ));
    }
    #[cfg(feature = "web")]
    {
        use crate::api::authentication::*;
        use crate::frontend::authenticate_passkey_js;
        use webauthn_rs_proto::*;

        status.set(Status::RequestingChallenge);
        let challenge = match start_authentication(name).await {
            Ok(c) => c,
            Err(e) => {
                return status.set(Status::ServerError(e.to_string()));
            }
        };

        let challenge_json = serde_json::to_string(&challenge).unwrap();
        match authenticate_passkey_js(&challenge_json).await {
            Ok(val) => {
                let res: PublicKeyCredential =
                    serde_json::from_str(&val.as_string().unwrap()).unwrap();
                match finish_authentication(res).await {
                    Ok(_) => status.set(Status::Success),
                    Err(e) => status.set(Status::ServerError(e.to_string())),
                }
            }
            Err(e) => {
                let err_msg = js_sys::Object::from(e)
                    .to_string()
                    .as_string()
                    .unwrap_or_default();
                status.set(Status::BrowserError(err_msg));
            }
        }
    }
}
