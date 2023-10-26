use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum DisplayStyle {
    ECMAScript = "ECMAScript",
    LazyK = "Lazy_K",
}
