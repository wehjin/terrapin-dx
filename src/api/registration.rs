use dioxus::fullstack::{server, ServerFnError};
use dioxus::prelude::*;
use uuid::Uuid;
use webauthn_rs_proto::*;

#[server]
pub async fn start_registration(
    user_id: Uuid,
    username: String,
) -> Result<CreationChallengeResponse, ServerFnError> {
    use crate::backend::passkey::*;
    use tower_sessions::Session;

    if !is_valid_username(&username) {
        return Err(ServerFnError::new(format!(
            "Invalid username: {}",
            username
        )));
    }
    if is_active_user(&username) {
        return Err(ServerFnError::new(format!(
            "Duplicate username: {}",
            username
        )));
    }
    let session: Session = FullstackContext::extract()
        .await
        .map_err(|_| ServerFnError::new("Failed to extract session"))?;

    let (challenge, registration_state) = WEBAUTHN
        .start_passkey_registration(user_id, &username, &username, None)
        .map_err(|e| ServerFnError::new(format!("WebAuthn Error: {}", e)))?;

    let context = RegistrationContext {
        passkey_registration: registration_state,
        user_id,
        username,
    };
    session
        .insert("reg_context", context)
        .await
        .map_err(|e| ServerFnError::new(format!("Session error: {}", e)))?;

    Ok(challenge)
}

#[server]
pub async fn finish_registration(
    reg_response: RegisterPublicKeyCredential,
) -> Result<(), ServerFnError> {
    use crate::backend::passkey::*;
    use tower_sessions::Session;

    let session: Session = FullstackContext::extract()
        .await
        .map_err(|_| ServerFnError::new("Failed to extract session"))?;
    let context: RegistrationContext = session
        .remove("reg_context")
        .await
        .map_err(|e| ServerFnError::new(format!("Session error: {}", e)))?
        .ok_or_else(|| ServerFnError::new("Registration session expired"))?;
    let passkey = WEBAUTHN
        .finish_passkey_registration(&reg_response, &context.passkey_registration)
        .map_err(|e| ServerFnError::new(format!("Verification failed: {e}")))?;
    register_user(Login {
        passkey,
        user_id: context.user_id,
        username: context.username.clone(),
    })
    .map_err(|e| ServerFnError::new(format!("Failed to register user: {e}")))?;
    Ok(())
}
