use crate::api::ecs::{Eid, LotItem};
use crate::data::market::Product;
use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod authentication;
pub mod ecs;
pub mod registration;
pub mod session;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub user_id: Uuid,
}

#[server]
pub async fn active_user() -> Result<Option<User>, ServerFnError> {
    use tower_sessions::Session;

    let session: Session = FullstackContext::extract().await?;
    let user = session
        .get::<User>("user")
        .await
        .map_err(|_| ServerFnError::new("Failed to get user from session"))?;
    Ok(user)
}

#[server]
pub async fn query_lots() -> Result<Vec<LotItem>, ServerFnError> {
    use crate::backend::require_ecs;
    let ecs = require_ecs().await?;
    Ok(ecs.query_lots())
}

#[server]
pub async fn drop_lot(eid: Eid) -> Result<(), ServerFnError> {
    use crate::api::session::fetch_session;
    let mut session = fetch_session()
        .await?
        .ok_or(ServerFnError::new("No session"))?;
    session
        .ecs
        .drop_lot(eid.clone())
        .map_err(|_| ServerFnError::new("Failed to drop lot"))?;
    info!("Dropped lot: {:?}", eid);
    Ok(())
}

#[server]
pub async fn query_products() -> Result<Vec<Product>, ServerFnError> {
    use crate::backend::require_ecs;
    let ecs = require_ecs().await?;
    Ok(ecs.query_products())
}

#[server]
pub async fn update_product_prices(csv: String) -> Result<(), ServerFnError> {
    use crate::backend::require_ecs;
    use crate::data::yf;
    info!("Importing prices");
    let market_prices = yf::parse_market_prices(csv.as_bytes())
        .map_err(|e| ServerFnError::new(format!("Failed to parse market prices: {}", e)))?;
    let mut ecs = require_ecs().await?;
    ecs.update_prices(market_prices)
        .map_err(|e| ServerFnError::new(format!("Failed to update prices: {}", e)))?;
    info!("Updated prices");
    Ok(())
}
