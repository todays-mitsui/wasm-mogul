use tuber::DisplayStyle;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = DisplayStyle)]
pub enum JsDisplayStyle {
    EcmaScript = "ECMAScript",
    LazyK = "Lazy_K",
}

#[wasm_bindgen(typescript_custom_section)]
const TS_DISPLAY_STYLE: &'static str = r#"
export type DisplayStyle = "ECMAScript" | "Lazy_K";
"#;

impl From<JsDisplayStyle> for DisplayStyle {
    fn from(js_display_style: JsDisplayStyle) -> DisplayStyle {
        match js_display_style {
            JsDisplayStyle::EcmaScript => DisplayStyle::EcmaScript,
            JsDisplayStyle::LazyK => DisplayStyle::LazyK,
            JsDisplayStyle::__Invalid => DisplayStyle::EcmaScript,
        }
    }
}

impl From<DisplayStyle> for JsDisplayStyle {
    fn from(display_style: DisplayStyle) -> JsDisplayStyle {
        match display_style {
            DisplayStyle::EcmaScript => JsDisplayStyle::EcmaScript,
            DisplayStyle::LazyK => JsDisplayStyle::LazyK,
        }
    }
}
