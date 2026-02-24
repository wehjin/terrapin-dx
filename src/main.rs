use dioxus::prelude::*;
use views::{Dev, Home, Navbar, Register, TestLogin};

pub mod api;
pub mod bulma;
mod components;
pub mod data;
mod views;

#[cfg(feature = "server")]
mod backend;
#[cfg(feature = "web")]
mod frontend;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

    #[route("/")]
    Home {},

    #[route("/dev")]
    Dev {},

    #[route("/register")]
    Register {},

    #[route("/login")]
    TestLogin {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
fn main() {
    #[cfg(feature = "web")]
    {
        launch(App);
    }
    #[cfg(feature = "server")]
    {
        use tower_sessions::cookie::time::Duration;
        use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
        let is_prod = std::env::var("APP_ENV").unwrap_or_default() == "production";
        info!("Running in production mode: {is_prod}");
        serve(|| async move {
            let session_store = MemoryStore::default();
            let session_layer = SessionManagerLayer::new(session_store)
                .with_secure(is_prod)
                .with_expiry(Expiry::OnInactivity(Duration::hours(1)));
            let router = dioxus::server::router(App).layer(session_layer);
            Ok(router)
        });
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0"}
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css" }
        Router::<Route> {}
    }
}
