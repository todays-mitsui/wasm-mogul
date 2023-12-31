use super::expression::ExprJson;
use super::JsDisplayStyle;
use serde::Serialize;
use tuber::{Format, Func};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Func)]
pub struct JsFunc(Func);

#[wasm_bindgen(js_class = Func)]
impl JsFunc {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn params(&self) -> Box<[JsValue]> {
        self.0
            .params()
            .iter()
            .map(|id| JsValue::from_str(id.as_str()))
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.0.body().to_string()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen]
    pub fn format(&self, display_style: JsDisplayStyle) -> String {
        self.0.format(&display_style.into())
    }
}

impl From<Func> for JsFunc {
    fn from(func: Func) -> JsFunc {
        JsFunc(func)
    }
}

impl From<JsFunc> for Func {
    fn from(js_func: JsFunc) -> Func {
        js_func.0
    }
}

// ========================================================================== //

#[derive(Serialize)]
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
