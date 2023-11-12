use crate::js_value::{JsCommand, JsExpr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_command(input: &str) -> Result<JsCommand, JsError> {
    match tuber::parse_command(input) {
        Ok(command) => Ok(command.into()),
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}

#[wasm_bindgen]
pub fn parse_expr(input: &str) -> Result<JsExpr, JsError> {
    match tuber::parse_expr(input) {
        Ok(expr) => Ok(expr.into()),
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}
