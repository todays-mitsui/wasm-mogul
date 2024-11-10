use crate::context::Context;
use crate::expression::Expr;
use js_sys::Symbol;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use tuber::Eval;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Reducer(Eval);

#[wasm_bindgen]
impl Reducer {
    #[wasm_bindgen(constructor)]
    pub fn new(context: Context, expr: Expr) -> Self {
        let tuber_context = context.into();
        let tuber_expr = expr.into();
        Self(Eval::new(tuber_context, tuber_expr))
    }

    // #[wasm_bindgen(js_name = next)]
    // pub fn js_next(&mut self) -> JsNext {}

    pub fn reducible(&self) -> bool {
        self.0.next_path().is_some()
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct JsNext {
    done: bool,
    value: Option<Expr>,
}
