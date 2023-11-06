mod evaluate;

use tuber::Identifier;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Identifier)]
pub struct JsIdentifier(Identifier);

#[wasm_bindgen(js_class = Identifier)]
impl JsIdentifier {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        Self(name.into())
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.as_str().to_string()
    }
}
