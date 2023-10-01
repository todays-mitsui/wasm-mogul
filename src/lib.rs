#[macro_use]
mod browser;
mod calc;
mod command;
mod context;
mod expr;
mod func;
mod parser;
mod to_string;

use calc::{unlambda_shallow, Eval, EvalStep};
use context::Context;
use parser::parse_expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(src: &str) {
    let result = parse_expr(src);
    log!("{:?}", result);
}

#[wasm_bindgen]
pub fn unlambda(src: &str) {
    let expr = parse_expr(src).expect("parse error");
    log!("{}", unlambda_shallow(expr));
}

#[wasm_bindgen]
pub fn eval_(src: &str) {
    let expr = parse_expr(src).expect("parse error");
    let context = Context::default();

    let eval = Eval::new(expr, &context);

    for EvalStep { expr } in eval.take(1000) {
        log!("{}", expr);
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}
