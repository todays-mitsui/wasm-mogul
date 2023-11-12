use crate::js_value::{JsCommand, JsContext, JsRunResult};
use crate::repository::{get_context, push_history_def, push_history_del};
use tuber::Command;
use tuber::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(context: JsContext, command: JsCommand) -> Result<JsRunResult, JsError> {
    if let Command::Del(id) = command.as_ref() {
        push_history_del(id).map_err(|err| JsError::new(&err.to_string()))?;
    }

    if let Command::Update(func) = command.as_ref() {
        push_history_def(func).map_err(|err| JsError::new(&err.to_string()))?;
    }

    let engine = Engine::new(context.into());
    let result = engine.run(command.into());

    Ok(result.into())
}
