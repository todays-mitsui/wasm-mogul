use tuber::parse_expr;
use tuber::Expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn expr(input: &str) -> JsExpr {
    let expr = parse_expr(input).expect("parse error");
    JsExpr(expr)
}

#[wasm_bindgen(js_name = Expr)]
pub struct JsExpr(Expr);
