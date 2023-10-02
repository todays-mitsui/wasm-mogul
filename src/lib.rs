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
use parser::{parse_expr, parse_expr_with_ecmascript_style};
use to_string::ECMAScriptStyle;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Event};

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
pub fn lambda_calculus(src: &str) {
    let document = browser::document().unwrap();
    let container = browser::container().unwrap();

    let expr = parse_expr_with_ecmascript_style(src).expect("parse error");

    let p = document.create_element("p").unwrap();
    p.set_text_content(Some(ECMAScriptStyle(&expr).to_string().as_str()));
    container.append_child(&p).unwrap();

    let context = Context::default();
    let eval = Eval::new(expr, &context);

    for EvalStep { expr } in eval.take(1000) {
        log!("{}", ECMAScriptStyle(&expr));
        let p = document.create_element("p").unwrap();
        p.set_text_content(Some(ECMAScriptStyle(&expr).to_string().as_str()));
        container.append_child(&p).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}
