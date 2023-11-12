use crate::js_value::JsDisplayStyle;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getDisplayStyle)]
pub fn get_display_style() -> Result<JsDisplayStyle, JsError> {
    crate::repository::get_display_style()
        .map(|display_style| display_style.into())
        .map_err(|err| JsError::new(&err.to_string()))
}

#[wasm_bindgen(js_name = setDisplayStyle)]
pub fn set_display_style(display_style: JsDisplayStyle) {
    unimplemented!();
    // crate::repository::set_display_style(display_style.into());
}
