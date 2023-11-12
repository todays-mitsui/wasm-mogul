use super::JsDisplayStyle;
use serde::Serialize;
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

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&ExprJson::from(self.0.clone())).unwrap()
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

// ========================================================================== //

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExprJson {
    Variable {
        identifier: String,
    },
    Symbol {
        identifier: String,
    },
    Apply {
        lhs: Box<ExprJson>,
        rhs: Box<ExprJson>,
    },
    Lambda {
        param: String,
        body: Box<ExprJson>,
    },
}

impl From<Expr> for ExprJson {
    fn from(expr: Expr) -> ExprJson {
        match expr {
            Expr::Variable(name) => ExprJson::Variable {
                identifier: name.as_ref().to_string(),
            },
            Expr::Symbol(name) => ExprJson::Symbol {
                identifier: name.as_ref().to_string(),
            },
            Expr::Apply { lhs, rhs } => ExprJson::Apply {
                lhs: Box::new((*lhs).clone().into()),
                rhs: Box::new((*rhs).clone().into()),
            },
            Expr::Lambda { param, body } => ExprJson::Lambda {
                param: param.as_ref().to_string(),
                body: Box::new((*body).clone().into()),
            },
        }
    }
}
