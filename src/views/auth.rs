use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Register() -> Element {
    let user_id = Uuid::new_v4();
    let username = "Clark Kent".to_string();
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
            #[cfg(not(feature = "server"))]
            {
                use crate::api::auth::*;
                use frontend::register_passkey_js;
                use webauthn_rs_proto::*;

                status.set("Fetching challenge…".to_string());
                let challenge = match crate::api::auth::start_registration(id, name).await {
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
                        let err_msg = err
                            .as_string()
                            .unwrap_or_else(|| "Unknown Browser Error".into());
                        status.set(format!("Browser Error: {err_msg}"));
                    }
                }
            }
            #[cfg(feature = "server")]
            status.set(format!(
                "WebAuthn is only available in the browser: {id}, {name}"
            ));
        });
    };
    rsx! {
        h1 { class: "title", "Register" }
        h2 { class: "subtitle", "Register a Passkey" }
        button { class: "button",
            onclick: register,
            "Register Passkey"
        }
        p { "Status: {status}"}
    }
}

#[cfg(not(feature = "server"))]
mod frontend {
    use wasm_bindgen::JsValue;
    #[wasm_bindgen::prelude::wasm_bindgen(module = "/assets/webauthn.js")]
    extern "C" {
        #[wasm_bindgen::prelude::wasm_bindgen(catch)]
        pub async fn register_passkey_js(
            challenge_json: &str,
        ) -> std::result::Result<JsValue, JsValue>;
    }
}
