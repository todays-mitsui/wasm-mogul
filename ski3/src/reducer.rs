use crate::context::{to_tuber_context, Context};
use crate::expression::Expr;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use tsify_next::Tsify;
use tuber;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Reducer(tuber::Reducer);

#[wasm_bindgen]
impl Reducer {
    #[wasm_bindgen(constructor)]
    pub fn new(context: JsValue, expr: Expr) -> Self {
        let context: Context =
            serde_wasm_bindgen::from_value(context).expect("Failed to parse context");
        let tuber_context = to_tuber_context(context);
        let tuber_expr = expr.into();
        Self(tuber::Reducer::new(tuber_context, tuber_expr))
    }

    pub fn reducible_path(&self) -> Option<tuber::Path> {
        self.0.reducible_path()
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct JsNext {
    done: bool,
    value: Option<Expr>,
}
