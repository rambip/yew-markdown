use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/katex/katex.mjs")]
extern "C" {
    #[wasm_bindgen(js_namespace=default)]
    fn renderToString(_: &str, options: &JsValue) -> String;
}

pub fn render_to_string(input: &str) -> String {
    renderToString(input, &wasm_bindgen::JsValue::NULL)
}

pub static KATEX_CSS: &'static str = include_str!("../katex/katex.min.css");
