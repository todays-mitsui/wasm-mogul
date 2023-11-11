use super::{JsContext, JsExpr};
use tuber::{Context, Eval, Expr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Eval)]
pub struct JsEval(Eval);

#[wasm_bindgen(js_class = Eval)]
impl JsEval {
    #[wasm_bindgen(constructor)]
    pub fn new(context: JsContext, expr: JsExpr) -> Self {
        let context: Context = context.into();
        let expr: Expr = expr.into();

        Self(Eval::new(context, expr))
    }

    pub fn next(&mut self) -> Option<String> {
        self.0.next().map(|step| step.expr.to_string())
    }

    #[wasm_bindgen(js_name = hasNext)]
    pub fn has_next(&self) -> bool {
        self.0.clone().peekable().peek().is_some()
    }

    pub fn chunk(&mut self, size: usize) -> Box<[JsValue]> {
        let mut i = 0;
        let mut results = Vec::new();
        while i < size {
            if let Some(step) = self.0.next() {
                let js_value = JsValue::from_str(&step.expr.to_string());
                results.push(js_value);
            } else {
                break;
            }
            i = i + 1;
        }
        results.into_boxed_slice()
    }
}
