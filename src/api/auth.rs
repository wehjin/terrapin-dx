use dioxus::fullstack::{server, ServerFnError};
use dioxus::prelude::*;
use uuid::Uuid;
use webauthn_rs_proto::*;

#[server]
pub async fn start_registration(
    user_id: Uuid,
    username: String,
) -> Result<CreationChallengeResponse, ServerFnError> {
    #[cfg(target_arch = "wasm32")]
    return Err(ServerFnError::new("Backend only"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::backend::auth::WEBAUTHN;
        use tower_sessions::Session;

        let session: Session = FullstackContext::extract()
            .await
            .map_err(|_| ServerFnError::new("Failed to extract session"))?;

        let (challenge, registration_state) = WEBAUTHN
            .start_passkey_registration(user_id, &username, &username, None)
            .map_err(|e| ServerFnError::new(format!("WebAuthn Error: {}", e)))?;

        session
            .insert("reg_state", registration_state)
            .await
            .map_err(|e| ServerFnError::new(format!("Session error: {}", e)))?;

        Ok(challenge)
    }
}

#[server]
pub async fn finish_registration(
    reg_response: RegisterPublicKeyCredential,
) -> Result<(), ServerFnError> {
    use crate::backend::auth::WEBAUTHN;
    use tower_sessions::Session;
    use webauthn_rs::prelude::*;

    let session: Session = FullstackContext::extract()
        .await
        .map_err(|_| ServerFnError::new("Failed to extract session"))?;
    let state: PasskeyRegistration = session
        .remove("reg_state")
        .await
        .map_err(|e| ServerFnError::new(format!("Session error: {}", e)))?
        .ok_or_else(|| ServerFnError::new("Registration session expired"))?;
    let _passkey = WEBAUTHN
        .finish_passkey_registration(&reg_response, &state)
        .map_err(|e| ServerFnError::new(format!("Verification failed: {e}")))?;
    Ok(())
}
