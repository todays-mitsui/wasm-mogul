use super::{JsContext, JsDisplayStyle, JsExpr};
use serde::{Deserialize, Serialize};
use tuber::{
    ecmascript_format, lazy_k_format, Context, DisplayStyle, Eval, EvalStep, Expr, Path, Tag,
};
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

    pub fn next(&mut self, display_style: JsDisplayStyle) -> JsValue {
        match self.0.next() {
            None => serde_wasm_bindgen::to_value(&JsNextResult {
                value: None,
                done: true,
            })
            .unwrap(),
            Some(step) => serde_wasm_bindgen::to_value(&JsNextResult {
                value: Some(JsEvalStep::from((step, display_style.into()))),
                done: !self.has_next(),
            })
            .unwrap(),
        }
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

impl From<Eval> for JsEval {
    fn from(eval: Eval) -> JsEval {
        JsEval(eval)
    }
}

impl From<JsEval> for Eval {
    fn from(js_eval: JsEval) -> Eval {
        js_eval.0
    }
}

// ========================================================================== //

#[derive(Serialize, Deserialize)]
pub struct JsNextResult {
    pub value: Option<JsEvalStep>,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsEvalStep {
    pub step: usize,
    pub expr: String,
    pub reduced: Option<String>,
    pub next: Option<String>,
}

impl From<EvalStep> for JsEvalStep {
    fn from(step: EvalStep) -> JsEvalStep {
        let mut paths = vec![step.reduced_path.clone()];
        if let Some(next_path) = step.next_path.clone() {
            paths.push(next_path);
        }

        let formed = lazy_k_format(&step.expr);

        let reduced_range = reduced_path_to_range(&formed.mapping, &step.reduced_path);
        let next_range = step
            .next_path
            .and_then(|next_path| next_path_to_range(&formed.mapping, &next_path));

        JsEvalStep {
            step: step.step,
            expr: step.expr.to_string(),
            reduced: reduced_range,
            next: next_range,
        }
    }
}

impl From<(EvalStep, DisplayStyle)> for JsEvalStep {
    fn from((step, display_style): (EvalStep, DisplayStyle)) -> JsEvalStep {
        let mut paths = vec![step.reduced_path.clone()];
        if let Some(next_path) = step.next_path.clone() {
            paths.push(next_path);
        }

        let formed = match display_style {
            DisplayStyle::EcmaScript => ecmascript_format(&step.expr, &paths),
            DisplayStyle::LazyK => lazy_k_format(&step.expr),
        };

        let reduced_range = reduced_path_to_range(&formed.mapping, &step.reduced_path);
        let next_range = step
            .next_path
            .and_then(|next_path| next_path_to_range(&formed.mapping, &next_path));

        JsEvalStep {
            step: step.step,
            expr: formed.expr,
            reduced: reduced_range,
            next: next_range,
        }
    }
}

fn reduced_path_to_range(mapping: &[Tag], path: &Path) -> Option<String> {
    path.range(mapping)
        .map(|std::ops::Range { start, end }| format!("{},{}", start, end))
}

fn next_path_to_range(mapping: &[Tag], path: &Path) -> Option<String> {
    let arity: usize = path.get_arity();

    let mut callee_path = path.clone();
    callee_path.set_arity(0);

    let args_path = (0..arity).map(|index| {
        let mut path = path.clone();
        path.set_arity(index + 1);
        path.last_arg();
        path
    });

    let mut range_strs = vec![
        path.range(mapping)
            .map(|std::ops::Range { start, end }| format!("{},{}", start, end))?,
        callee_path
            .range(mapping)
            .map(|std::ops::Range { start, end }| format!("{},{}", start, end))?,
    ];

    for arg_path in args_path {
        let arg_range_str = arg_path
            .range(mapping)
            .map(|std::ops::Range { start, end }| format!("{},{}", start, end));
        if let Some(arg_range_str) = arg_range_str {
            range_strs.push(arg_range_str);
        }
    }

    Some(range_strs.join(";"))
}
