use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum DisplayStyle {
    EcmaScript,
    LazyK,
}

impl From<DisplayStyle> for tuber::DisplayStyle {
    fn from(ski_display_style: DisplayStyle) -> tuber::DisplayStyle {
        match ski_display_style {
            DisplayStyle::EcmaScript => tuber::DisplayStyle::EcmaScript,
            DisplayStyle::LazyK => tuber::DisplayStyle::LazyK,
        }
    }
}

impl From<tuber::DisplayStyle> for DisplayStyle {
    fn from(tuber_display_style: tuber::DisplayStyle) -> DisplayStyle {
        match tuber_display_style {
            tuber::DisplayStyle::EcmaScript => DisplayStyle::EcmaScript,
            tuber::DisplayStyle::LazyK => DisplayStyle::LazyK,
        }
    }
}

impl From<&tuber::DisplayStyle> for DisplayStyle {
    fn from(tuber_display_style: &tuber::DisplayStyle) -> DisplayStyle {
        match tuber_display_style {
            tuber::DisplayStyle::EcmaScript => DisplayStyle::EcmaScript,
            tuber::DisplayStyle::LazyK => DisplayStyle::LazyK,
        }
    }
}
