use tuber::{Context, Eval, Expr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Eval)]
pub struct JsEval<'a>(Eval<'a>);

#[wasm_bindgen(js_class = Eval)]
impl JsEval<'a> {
    pub fn new(expr: Expr, context: &Context) -> Self {
        Self(Eval::new(expr, context))
    }

    fn next(&mut self) -> Option<JsEvalStep> {
        self.0.next().map(JsEvalStep::from)
    }
}
