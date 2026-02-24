use wasm_bindgen::JsValue;
#[wasm_bindgen::prelude::wasm_bindgen(module = "/assets/webauthn.js")]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(catch)]
    pub async fn register_passkey_js(challenge_json: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen::prelude::wasm_bindgen(catch)]
    pub async fn authenticate_passkey_js(challenge_json: &str) -> Result<JsValue, JsValue>;
}
