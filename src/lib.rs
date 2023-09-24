#[macro_use]
mod browser;
mod command;
mod context;
mod expr;
mod func;
mod parser;
mod to_string;

use parser::parse_expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(src: &str) {
    let result = parse_expr(src);
    log!("{:?}", result);
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let e = expr::l("x", expr::a("x", ":y"));
    log!("{:}", e);

    let i = "i".to_string();
    let f = func::new(i, vec!["x"], "x");
    log!("{:}", f);

    Ok(())
}
