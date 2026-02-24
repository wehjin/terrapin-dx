use crate::data::market::Product;
use crate::data::portfolio::Lot;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub login_name: String,
    pub products: Vec<Product>,
    pub lots: Vec<Lot>,
}

#[server]
pub async fn fetch_session() -> Result<Option<SessionState>, ServerFnError> {
    use crate::api::active_user;
    use crate::backend::session::load_session;

    match active_user().await? {
        None => Ok(None),
        Some(user) => {
            let session = load_session(user.username).await?;
            Ok(Some(session))
        }
    }
}
