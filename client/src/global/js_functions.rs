use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    /// Used for logging state updates as JavaScript objects (to prevent unnecessarily long, stringified logs)
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_log_with_styling(label: &str, styling: &str, object: &JsValue);
}
