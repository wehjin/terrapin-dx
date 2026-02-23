use std::sync::LazyLock;
use webauthn_rs::prelude::*;

pub static WEBAUTHN: LazyLock<Webauthn> = LazyLock::new(|| {
    let rp_id = std::env::var("WEBAUTHN_RP_ID").unwrap_or_else(|_| "localhost".to_string());
    let rp_origin_url =
        std::env::var("WEBAUTHN_RP_ORIGIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let rp_origin = url::Url::parse(&rp_origin_url).expect("Invalid RP Origin URL");
    WebauthnBuilder::new(&rp_id, &rp_origin)
        .expect("Invalid RP Configuration")
        .build()
        .expect("Failed to build Webauthn")
});
