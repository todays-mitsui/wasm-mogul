#[macro_use]
mod browser;
mod display_style;
mod execute;
mod js_value;
mod repository;
mod style;
mod util;

pub use display_style::{get_display_style, set_display_style};
pub use execute::execute;
pub use js_value::{JsCommand, JsContext, JsEval};
pub use util::{parse_command, parse_expr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}
