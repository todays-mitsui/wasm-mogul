use super::JsDisplayStyle;
use tuber::{parse_expr, Expr, Format};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Expr)]
pub struct JsExpr(Expr);

#[wasm_bindgen(js_class = Expr)]
impl JsExpr {
    #[wasm_bindgen]
    pub fn parse(input: &str) -> Result<JsExpr, JsError> {
        let expr = parse_expr(input).map_err(|_err| JsError::new("parse error"))?;
        Ok(JsExpr(expr))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen]
    pub fn format(&self, display_style: JsDisplayStyle) -> String {
        self.0.format(&display_style.into())
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
