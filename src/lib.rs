#[macro_use]
mod browser;
mod calc;
mod command;
mod context;
mod expr;
mod func;
mod parser;
mod to_string;

use calc::{Eval, EvalStep};
use context::Context;
use parser::{parse_expr_with_ecmascript_style, parse_expr_with_lazy_k_style};
use to_string::{ECMAScriptStyle, LazyKStyle};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Style {
    ECMAScript = "ECMAScript",
    LazyK = "Lazy_K",
}

#[wasm_bindgen(getter_with_clone)]
pub struct CalcResult {
    pub expr: String,
    pub steps: Box<[JsValue]>,
}

#[wasm_bindgen]
pub fn lambda_calculus(src: &str, style: Style) -> CalcResult {
    let expr = match style {
        Style::ECMAScript => parse_expr_with_ecmascript_style(src).expect("parse error"),
        Style::LazyK => parse_expr_with_lazy_k_style(src).expect("parse error"),
        _ => unreachable!(),
    };

    let context = Context::default();
    let eval = Eval::new(expr.clone(), &context);

    let steps: Box<[JsValue]> = eval
        .take(1000)
        .map(|EvalStep { expr }| {
            match style {
                Style::ECMAScript => ECMAScriptStyle(&expr).to_string(),
                Style::LazyK => LazyKStyle(&expr).to_string(),
                _ => unreachable!(),
            }
            .into()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    return CalcResult {
        expr: match style {
            Style::ECMAScript => ECMAScriptStyle(&expr).to_string(),
            Style::LazyK => LazyKStyle(&expr).to_string(),
            _ => unreachable!(),
        },
        steps,
    };
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}
