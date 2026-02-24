use crate::api::registration::finish_registration;
use crate::components::error::ErrorMessage;
use crate::components::status::StatusMessage;
use dioxus::prelude::*;
use webauthn_rs_proto::RegisterPublicKeyCredential;

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Ready,
    FetchingChallenge,
    WaitingForBiometricPrompt,
    Success,
    ServerError(String),
    VerificationFailed(String),
    BrowserError(String),
}

#[component]
pub fn Register() -> Element {
    let mut status = use_signal(|| Status::Ready);
    let mut username = use_signal(|| "".to_string());
    let mut register = use_action(move || async move {
        let username = username();
        register_passkey(username, status.clone()).await;
        Ok::<(), anyhow::Error>(())
    });
    rsx! {
        div { class: "section",
            h1 { class: "title", "Register" }
            match status() {
                Status::Ready => rsx!(
                    div { class: "field",
                        label { class: "label", "Username" }
                        div { class: "control",
                            input { class: "input", value: "{username}", autocomplete: "username", autofocus: true,
                                oninput: move |e| {
                                    username.set(e.value());
                                },
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        register.call();
                                    }
                                }
                            }
                        }
                    }
                    button { class: "button is-primary",
                        onclick: move |_| {
                            register.call();
                        },
                        "Register Passkey"
                    }
                ),
                Status::FetchingChallenge => rsx!(StatusMessage{message: "Fetching challenge"}),
                Status::WaitingForBiometricPrompt => rsx!(StatusMessage{message: "Waiting for biometric prompt"}),
                Status::Success => rsx!(StatusMessage { message: "Success!"}),
                Status::ServerError(e) => rsx!(
                    ErrorMessage{error_type: "Server Error", message: e, onretry: move |_| status.set(Status::Ready) }
                ),
                Status::VerificationFailed(e) => rsx!(
                    ErrorMessage { error_type: "Verification Failed", message: e,onretry: move |_| status.set(Status::Ready)}
                ),
                Status::BrowserError(e) => rsx!(
                    ErrorMessage { error_type: "Browser Error", message: e, onretry: move |_| status.set(Status::Ready)}
                ),
            }
        }
    }
}

async fn register_passkey(name: String, mut status: Signal<Status>) {
    #[cfg(not(feature = "web"))]
    {
        status.set(Status::ServerError(
            "This platform does not support WebAuthn.".into(),
        ));
    }
    #[cfg(feature = "web")]
    {
        use crate::frontend;

        let id = uuid::Uuid::new_v4();
        status.set(Status::FetchingChallenge);
        let challenge = match crate::api::registration::start_registration(id, name).await {
            Ok(c) => c,
            Err(e) => {
                return status.set(Status::ServerError(e.to_string()));
            }
        };
        status.set(Status::WaitingForBiometricPrompt);
        let challenge_json = serde_json::to_string(&challenge).unwrap();
        match frontend::register_passkey_js(&challenge_json).await {
            Ok(js_val) => {
                let js_str = js_val.as_string().unwrap();
                let reg_result: RegisterPublicKeyCredential =
                    serde_json::from_str(&js_str).unwrap();
                match finish_registration(reg_result).await {
                    Ok(_) => status.set(Status::Success),
                    Err(e) => status.set(Status::VerificationFailed(e.to_string())),
                }
            }
            Err(err) => {
                let err_msg = js_sys::Object::from(err)
                    .to_string() // Calls the JS .toString() method
                    .as_string() // Converts the resulting JsString to Rust String
                    .unwrap_or_else(|| "Unknown Browser Error".into());
                status.set(Status::BrowserError(err_msg));
            }
        }
    }
}
