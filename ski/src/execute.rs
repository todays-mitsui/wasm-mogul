use crate::js_value::{JsCommand, JsContext, JsDisplayStyle, JsRunResult};
use crate::repository::{push_history_def, push_history_del};
use tuber::Command;
use tuber::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(
    context: JsContext,
    command: JsCommand,
    display_style: JsDisplayStyle,
) -> Result<JsRunResult, JsError> {
    if let Command::Del(id) = command.as_ref() {
        push_history_del(id).map_err(|err| JsError::new(&err.to_string()))?;
    }

    if let Command::Update(func) = command.as_ref() {
        push_history_def(func).map_err(|err| JsError::new(&err.to_string()))?;
    }

    let display_style = display_style.into();
    let engine = Engine::new(context.into());
    let result = engine.run(command.into());

    Ok((result, display_style).into())
}
