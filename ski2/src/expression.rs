use crate::display_style::JsDisplayStyle;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use tuber::{DisplayStyle, Expr, Format};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
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

impl From<ExprJson> for Expr {
    fn from(expr_json: ExprJson) -> Expr {
        match expr_json {
            ExprJson::Variable { identifier } => Expr::Variable(identifier.into()),
            ExprJson::Symbol { identifier } => Expr::Symbol(identifier.into()),
            ExprJson::Apply { lhs, rhs } => Expr::Apply {
                lhs: Box::new((*lhs).into()),
                rhs: Box::new((*rhs).into()),
            },
            ExprJson::Lambda { param, body } => Expr::Lambda {
                param: param.into(),
                body: Box::new((*body).into()),
            },
        }
    }
}

#[wasm_bindgen(js_name = parseExpr)]
pub fn parse_expr(input: &str) -> Result<JsValue, JsError> {
    match tuber::parse_expr(input) {
        Ok(expr) => {
            let expr_json: ExprJson = expr.into();
            Ok(serde_wasm_bindgen::to_value(&expr_json).unwrap())
        }
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}

#[wasm_bindgen(js_name = formatExpr)]
pub fn format_expr(js_obj: JsValue, js_display_style: JsDisplayStyle) -> Result<String, JsError> {
    let expr_json: ExprJson = from_value(js_obj).map_err(|e| JsError::new(&e.to_string()))?;
    let expr: Expr = expr_json.into();
    let display_style: DisplayStyle = js_display_style.into();
    Ok(expr.format(&display_style))
}
