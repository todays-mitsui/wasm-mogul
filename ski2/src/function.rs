use super::expression::ExprJson;
use crate::display_style::JsDisplayStyle;
use serde::{Deserialize, Serialize};
use tuber::{Format, Func};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct FuncJson {
    name: String,
    params: Vec<String>,
    body: ExprJson,
}

impl From<Func> for FuncJson {
    fn from(func: Func) -> FuncJson {
        FuncJson {
            name: func.name().to_string(),
            params: func
                .params()
                .iter()
                .map(|id| id.as_str().to_string())
                .collect(),
            body: ExprJson::from(func.body().clone()),
        }
    }
}

impl From<FuncJson> for Func {
    fn from(func_json: FuncJson) -> Func {
        Func::new(
            func_json.name.into(),
            func_json.params.into_iter().map(|id| id.into()).collect(),
            func_json.body.into(),
        )
    }
}
