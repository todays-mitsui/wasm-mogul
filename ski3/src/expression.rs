use crate::display_style::DisplayStyle;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::{self, Format};
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Expr {
    /* Variable */ V(Identifier),
    /* Symbol   */ S(Identifier),
    /* Apply    */ A(Box<Expr>, Box<Expr>),
    /* Lambda   */ L(Identifier, Box<Expr>),
}

impl From<tuber::Expr> for Expr {
    fn from(tuber_expr: tuber::Expr) -> Expr {
        match tuber_expr {
            tuber::Expr::Variable(name) => Expr::V(name.as_ref().to_string()),
            tuber::Expr::Symbol(name) => Expr::S(name.as_ref().to_string()),
            tuber::Expr::Apply { lhs, rhs } => Expr::A(
                Box::new((*lhs).clone().into()),
                Box::new((*rhs).clone().into()),
            ),
            tuber::Expr::Lambda { param, body } => {
                Expr::L(param.as_ref().to_string(), Box::new((*body).clone().into()))
            }
        }
    }
}

impl From<Expr> for tuber::Expr {
    fn from(ski_expr: Expr) -> tuber::Expr {
        match ski_expr {
            Expr::V(identifier) => tuber::Expr::Variable(identifier.into()),
            Expr::S(identifier) => tuber::Expr::Symbol(identifier.into()),
            Expr::A(lhs, rhs) => tuber::Expr::Apply {
                lhs: Box::new((*lhs).into()),
                rhs: Box::new((*rhs).into()),
            },
            Expr::L(param, body) => tuber::Expr::Lambda {
                param: param.into(),
                body: Box::new((*body).into()),
            },
        }
    }
}

#[wasm_bindgen(js_name = parseExpr)]
pub fn parse_expr(input: &str) -> Result<Expr, JsError> {
    match tuber::parse_expr(input) {
        Ok(expr) => Ok(expr.into()),
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}

#[wasm_bindgen(js_name = renderExpr)]
#[allow(non_snake_case)]
pub fn render_expr(expr: Expr, displayStyle: DisplayStyle) -> String {
    let tuber_expr: tuber::Expr = expr.into();
    let tuber_display_style: tuber::DisplayStyle = displayStyle.into();
    tuber_expr.format(&tuber_display_style)
}

// #[wasm_bindgen(js_name = formatExpr)]
// pub fn format_expr(expr: Expr, display_style: DisplayStyle) -> String {
//     let tuber_expr: tuber::Expr = expr.into();
//     let tuber_display_style: tuber::DisplayStyle = display_style.into();

//     match tuber_display_style {
//         tuber::DisplayStyle::EcmaScript => tuber::ecmascript_format(&tuber_expr),
//         tuber::DisplayStyle::LazyK => tuber_expr.format(&tuber_display_style),
//     }

//     tuber_expr.format(&tuber_display_style)
// }

// struct FormatOption {
//     display_style: DisplayStyle,
//     reducible_path: tuber::Path,
//     splits: Vec<tuber::Path>,
// }
