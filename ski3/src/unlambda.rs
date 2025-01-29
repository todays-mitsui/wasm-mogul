use crate::{Context, Expr};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn expand(context: Context, expr: Expr) -> Expr {
    let tuber_context = tuber::Context::from(context);
    let tuber_expr = tuber::Expr::from(expr);
    tuber::expand(&tuber_context, tuber_expr).into()
}

#[wasm_bindgen(js_name = unlambdaRecursive)]
pub fn unlambda_recursive(context: Context, expr: Expr) -> Expr {
    let tuber_context = tuber::Context::from(context);
    let tuber_expr = tuber::Expr::from(expr);
    tuber::unlambda_recursive(&tuber_context, tuber_expr).into()
}

#[wasm_bindgen(js_name = unlambdaRecursive_)]
pub fn unlambda_recursive_(context: Context, expr: Expr) -> Expr {
    let tuber_context = tuber::Context::from(context);
    let tuber_expr = tuber::Expr::from(expr);
    tuber::unlambda_recursive_(&tuber::RecursiveStrategy::SK, &tuber_context, tuber_expr).into()
}

#[wasm_bindgen(js_name = unlambdaIota)]
pub fn unlambda_iota(context: Context, expr: Expr) -> Expr {
    let tuber_context = tuber::Context::from(context);
    let tuber_expr = tuber::Expr::from(expr);
    tuber::unlambda_iota(&tuber_context, tuber_expr).into()
}
