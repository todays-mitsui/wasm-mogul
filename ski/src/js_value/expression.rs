use tuber::{parse_expr, Expr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Expr)]
pub struct JsExpr(Expr);

#[wasm_bindgen(js_class = Expr)]
impl JsExpr {
    #[wasm_bindgen]
    pub fn parse(input: &str) -> Option<JsExpr> {
        let expr = parse_expr(input).ok()?;
        Some(JsExpr(expr))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Expr> for JsExpr {
    fn from(expr: Expr) -> JsExpr {
        JsExpr(expr)
    }
}

impl From<JsExpr> for Expr {
    fn from(js_expr: JsExpr) -> Expr {
        js_expr.0
    }
}
