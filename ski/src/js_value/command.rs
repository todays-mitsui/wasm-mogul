use tuber::{parse_command, Command};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Command)]
pub struct JsCommand(Command);

#[wasm_bindgen(js_class = Command)]
impl JsCommand {
    pub fn parse(input: &str) -> Result<JsCommand, JsError> {
        let command = parse_command(input).map_err(|_err| JsError::new("parse error"))?;
        Ok(JsCommand(command))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Command> for JsCommand {
    fn from(command: Command) -> JsCommand {
        JsCommand(command)
    }
}

impl From<JsCommand> for Command {
    fn from(js_command: JsCommand) -> Command {
        js_command.0
    }
}

impl AsRef<Command> for JsCommand {
    fn as_ref(&self) -> &Command {
        &self.0
    }
}
