mod utils;

pub use mesc::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn is_mesc_enabled_wrapper() -> bool {
    mesc::is_mesc_enabled()
}

#[wasm_bindgen]
pub fn get_default_endpoint_wrapper(option: Option<String>) -> Result<JsValue, JsValue> {
    let option = option.as_deref();
    let result =
        mesc::get_default_endpoint(option).map_err(|e| JsValue::from_str(&e.to_string()))?;
    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
