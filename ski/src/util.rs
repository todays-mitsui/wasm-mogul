use crate::js_value::{JsCommand, JsExpr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_command(input: &str) -> Result<JsValue, JsError> {
    match tuber::parse_command(input) {
        Ok(command) => {
            let js_command: JsCommand = command.into();
            Ok(js_command.to_json())
        }
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}

#[wasm_bindgen(js_name = parseCommand)]
pub fn parse_command_(input: &str) -> Result<JsValue, JsError> {
    match tuber::parse_command(input) {
        Ok(command) => {
            let js_command: JsCommand = command.into();
            Ok(js_command.to_json())
        }
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}

#[wasm_bindgen]
pub fn parse_expr(input: &str) -> Result<JsValue, JsError> {
    match tuber::parse_expr(input) {
        Ok(expr) => {
            let js_expr: JsExpr = expr.into();
            Ok(js_expr.to_json())
        }
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}

#[wasm_bindgen(js_name = parseExpr)]
pub fn parse_expr_(input: &str) -> Result<JsValue, JsError> {
    match tuber::parse_expr(input) {
        Ok(expr) => {
            let js_expr: JsExpr = expr.into();
            Ok(js_expr.to_json())
        }
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}
