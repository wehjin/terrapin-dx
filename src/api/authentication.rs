
use dioxus::fullstack::{server, ServerFnError};
use dioxus::prelude::*;
use webauthn_rs_proto::*;

#[server]
pub async fn start_authentication(
    username: String,
) -> Result<RequestChallengeResponse, ServerFnError> {
    use crate::backend::passkey::*;
    use tower_sessions::Session;

    let session: Session = FullstackContext::extract().await?;
    let login = load_login(&username)
        .map_err(|e| ServerFnError::new(format!("Failed to load user: {e}")))?;
    let (challenge, passkey_authentication) = WEBAUTHN
        .start_passkey_authentication(&vec![login.passkey])
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let auth_context = AuthenticationContext {
        passkey_authentication,
        username,
        user_id: login.user_id,
    };
    session
        .insert("auth_context", auth_context)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(challenge)
}

#[server]
pub async fn finish_authentication(
    auth_response: PublicKeyCredential,
) -> Result<(), ServerFnError> {
    use crate::api::User;
    use crate::backend::passkey::*;
    use tower_sessions::Session;

    let session: Session = FullstackContext::extract().await?;
    let context = session
        .remove::<AuthenticationContext>("auth_context")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Authentication session expired"))?;
    let _auth_result = WEBAUTHN
        .finish_passkey_authentication(&auth_response, &context.passkey_authentication)
        .map_err(|e| ServerFnError::new(format!("Auth failed: {e}")))?;

    let user = User {
        username: context.username,
        user_id: context.user_id,
    };
    session
        .insert("user", user)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

