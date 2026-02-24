use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Register() -> Element {
    let user_id = Uuid::new_v4();
    let username = "clark_kent".to_string();
    rsx! {
        PasskeyRegistration { user_id, username }
    }
}

#[component]
fn PasskeyRegistration(user_id: Uuid, username: String) -> Element {
    let mut status = use_signal(|| "Ready".to_string());
    let register = move |_| {
        let id = user_id;
        let name = username.clone();
        spawn(async move {
            #[cfg(feature = "server")]
            {
                status.set(format!(
                    "WebAuthn is only available in the browser: {name}, {id}"
                ));
            }
            #[cfg(not(feature = "server"))]
            {
                use crate::api::registration::*;
                use crate::frontend::register_passkey_js;
                use webauthn_rs_proto::*;

                status.set("Fetching challenge…".to_string());
                let challenge = match crate::api::registration::start_registration(id, name).await {
                    Ok(c) => c,
                    Err(e) => {
                        status.set(format!("Server Error: {e}"));
                        return;
                    }
                };
                status.set("Waiting for biometric prompt…".to_string());
                let challenge_json = serde_json::to_string(&challenge).unwrap();
                match register_passkey_js(&challenge_json).await {
                    Ok(js_val) => {
                        let js_str = js_val.as_string().unwrap();
                        let reg_result: RegisterPublicKeyCredential =
                            serde_json::from_str(&js_str).unwrap();
                        match finish_registration(reg_result).await {
                            Ok(_) => status.set("Success! Passkey registered.".to_string()),
                            Err(e) => status.set(format!("Verification failed: {e}")),
                        }
                    }
                    Err(err) => {
                        let err_msg = js_sys::Object::from(err)
                            .to_string() // Calls the JS .toString() method
                            .as_string() // Converts the resulting JsString to Rust String
                            .unwrap_or_else(|| "Unknown Browser Error".into());
                        status.set(format!("Browser Error: {err_msg}"));
                    }
                }
            }
        });
    };
    rsx! {
        h1 { class: "title", "Register" }
        button { class: "button",
            onclick: register,
            "Register Passkey"
        }
        p { "Status: {status}"}
    }
}
