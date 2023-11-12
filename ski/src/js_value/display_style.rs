use tuber::DisplayStyle;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum JsDisplayStyle {
    EcmaScript = "ECMAScript",
    LazyK = "Lazy_K",
}

impl From<JsDisplayStyle> for DisplayStyle {
    fn from(js_display_style: JsDisplayStyle) -> DisplayStyle {
        match js_display_style {
            JsDisplayStyle::EcmaScript => DisplayStyle::EcmaScript,
            JsDisplayStyle::LazyK => DisplayStyle::LazyK,
            JsDisplayStyle::__Nonexhaustive => unreachable!(),
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

impl AsRef<DisplayStyle> for JsDisplayStyle {
    fn as_ref(&self) -> &DisplayStyle {
        match self {
            JsDisplayStyle::EcmaScript => &DisplayStyle::EcmaScript,
            JsDisplayStyle::LazyK => &DisplayStyle::LazyK,
            JsDisplayStyle::__Nonexhaustive => unreachable!(),
        }
    }
}
