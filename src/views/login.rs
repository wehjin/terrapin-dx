use crate::api;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let username = "clark_kent";
    let mut user_loader = use_loader(move || async move { api::active_user().await })?;
    match user_loader() {
        Some(user) => rsx! {
            div {
                "Hello {user.username} with id: {user.user_id}"
            }
        },
        None => rsx!(PasskeyAuthentication {
            username,
            success: move |_| user_loader.restart(),
        }),
    }
}

#[component]
pub fn PasskeyAuthentication(username: String, success: EventHandler<()>) -> Element {
    let mut status = use_signal(|| "Ready".to_string());

    let on_login = move |_| {
        let name = username.clone();
        spawn(async move {
            #[cfg(feature = "server")]
            {
                status.set(format!("WebAuthn is only available in the browser: {name}"));
            }
            #[cfg(not(feature = "server"))]
            {
                use crate::api::authentication::*;
                use crate::frontend::authenticate_passkey_js;
                use webauthn_rs_proto::*;

                status.set("Requesting login...".into());
                let challenge = match start_authentication(name).await {
                    Ok(c) => c,
                    Err(e) => return status.set(format!("Server Error: {e}")),
                };

                let json = serde_json::to_string(&challenge).unwrap();
                match authenticate_passkey_js(&json).await {
                    Ok(val) => {
                        let res: PublicKeyCredential =
                            serde_json::from_str(&val.as_string().unwrap()).unwrap();
                        match finish_authentication(res).await {
                            Ok(_) => {
                                status.set("Successfully logged in!".into());
                                success.call(());
                            }
                            Err(e) => status.set(format!("Login failed: {e}")),
                        }
                    }
                    Err(e) => {
                        let err_msg = js_sys::Object::from(e)
                            .to_string()
                            .as_string()
                            .unwrap_or_default();
                        status.set(format!("Browser Error: {err_msg}"));
                    }
                }
            }
        });
    };

    rsx! {
        h1 { class: "title", "Login" }
        button { class: "button", onclick: on_login, "Login with Passkey" }
        p { "Status: {status}" }
    }
}
