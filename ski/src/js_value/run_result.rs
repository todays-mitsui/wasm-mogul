use super::{JsContext, JsEval, JsExpr, JsFunc};
use tuber::RunResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = RunResult)]
pub struct JsRunResult(RunResult);

#[wasm_bindgen(js_class = RunResult)]
impl JsRunResult {
    #[wasm_bindgen(getter, js_name = type)]
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
        match &self.0 {
            RunResult::Del { input, .. } => input.as_str().to_string(),
            RunResult::Update { input, .. } => input.to_string(),
            RunResult::Eval { input, .. } => input.to_string(),
            RunResult::Search { input, .. } => input.as_str().to_string(),
            RunResult::Context { .. } => String::from(""),
            RunResult::Unlambda { input, .. } => input.to_string(),
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
        JsRunResult(run_result)
    }
}

impl From<JsRunResult> for RunResult {
    fn from(js_run_result: JsRunResult) -> RunResult {
        js_run_result.0
    }
}
