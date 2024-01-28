use super::{JsContext, JsEval, JsExpr, JsFunc};
use tuber::{ecmascript_format, lazy_k_format, DisplayStyle, Format, RunResult};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = RunResult)]
pub struct JsRunResult(RunResult, DisplayStyle);

#[wasm_bindgen(js_class = RunResult)]
impl JsRunResult {
    #[wasm_bindgen(getter, js_name = commandType)]
    pub fn command_type(&self) -> String {
        match self.0 {
            RunResult::Del { .. } => String::from("del"),
            RunResult::Update { .. } => String::from("update"),
            RunResult::Eval { .. } => String::from("eval"),
            RunResult::Search { .. } => String::from("search"),
            RunResult::Context { .. } => String::from("context"),
            RunResult::Unlambda { .. } => String::from("unlambda"),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn input(&self) -> String {
        let display_style = self.1;
        match &self.0 {
            RunResult::Del { input, .. } => input.as_str().to_string(),
            RunResult::Update { input, .. } => input.format(&display_style),
            RunResult::Eval { input, .. } => input.format(&display_style),
            RunResult::Search { input, .. } => input.as_str().to_string(),
            RunResult::Context { .. } => String::from(""),
            RunResult::Unlambda { input, .. } => input.format(&display_style),
        }
    }

    #[wasm_bindgen(getter, js_name = delResult)]
    pub fn del_result(&self) -> Option<JsContext> {
        if let RunResult::Del { result, .. } = &self.0 {
            Some(result.clone().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = updateResult)]
    pub fn update_result(&self) -> Option<JsContext> {
        if let RunResult::Update { result, .. } = &self.0 {
            Some(result.clone().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = evalInputNext)]
    pub fn eval_input_next(&self) -> Option<String> {
        if let RunResult::Eval {
            input: expr, eval, ..
        } = &self.0
        {
            let next_path = eval.next_path();
            if let Some(path) = next_path {
                let display_style = self.1;
                let paths = vec![path.clone()];
                let formed = match display_style {
                    DisplayStyle::EcmaScript => ecmascript_format(&expr, &paths),
                    DisplayStyle::LazyK => lazy_k_format(&expr),
                };

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
                    path.range(&formed.mapping)
                        .map(|std::ops::Range { start, end }| format!("{},{}", start, end))?,
                    callee_path
                        .range(&formed.mapping)
                        .map(|std::ops::Range { start, end }| format!("{},{}", start, end))?,
                ];

                for arg_path in args_path {
                    let arg_range_str = arg_path
                        .range(&formed.mapping)
                        .map(|std::ops::Range { start, end }| format!("{},{}", start, end));
                    if let Some(arg_range_str) = arg_range_str {
                        range_strs.push(arg_range_str);
                    }
                }

                Some(range_strs.join(";"))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = evalResult)]
    pub fn eval_result(&self) -> Option<JsEval> {
        if let RunResult::Eval { eval, .. } = &self.0 {
            Some(eval.clone().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = searchResult)]
    pub fn search_result(&self) -> Option<JsFunc> {
        if let RunResult::Search { result, .. } = &self.0 {
            result.clone().map(|func| func.into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = contextResult)]
    pub fn context_result(&self) -> Option<JsContext> {
        if let RunResult::Context { result } = &self.0 {
            Some(result.clone().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = unlambdaResult)]
    pub fn unlambda_result(&self) -> Option<JsExpr> {
        if let RunResult::Unlambda { result, .. } = &self.0 {
            Some(result.clone().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter, js_name = unlambdaLevel)]
    pub fn unlambda_level(&self) -> Option<u8> {
        if let RunResult::Unlambda { level, .. } = &self.0 {
            Some(*level)
        } else {
            None
        }
    }
}

impl From<RunResult> for JsRunResult {
    fn from(run_result: RunResult) -> JsRunResult {
        JsRunResult(run_result, DisplayStyle::LazyK)
    }
}

impl From<JsRunResult> for RunResult {
    fn from(js_run_result: JsRunResult) -> RunResult {
        js_run_result.0
    }
}

impl From<(RunResult, DisplayStyle)> for JsRunResult {
    fn from((run_result, display_style): (RunResult, DisplayStyle)) -> JsRunResult {
        JsRunResult(run_result, display_style)
    }
}
