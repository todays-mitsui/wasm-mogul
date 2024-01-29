use super::{expression::ExprJson, function::FuncJson};
use serde::Serialize;
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

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&CommandJson::from(self.0.clone())).unwrap()
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

// ========================================================================== //

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum CommandJson {
    Delete { identifier: String },
    Update { func: FuncJson },
    Evaluate { expr: ExprJson },
    EvaluateLast { expr: ExprJson },
    EvaluateHead { count: usize, expr: ExprJson },
    EvaluateTail { count: usize, expr: ExprJson },
    Search { identifier: String },
    Context,
    Unlambda { level: u8, expr: ExprJson },
}

impl From<Command> for CommandJson {
    fn from(command: Command) -> CommandJson {
        match command {
            Command::Del(id) => CommandJson::Delete {
                identifier: id.as_ref().to_string(),
            },
            Command::Update(func) => CommandJson::Update {
                func: FuncJson::from(func),
            },
            Command::Eval(expr) => CommandJson::Evaluate {
                expr: ExprJson::from(expr),
            },
            Command::EvalLast(expr) => CommandJson::EvaluateLast {
                expr: ExprJson::from(expr),
            },
            Command::EvalHead(count, expr) => CommandJson::EvaluateHead {
                count,
                expr: ExprJson::from(expr),
            },
            Command::EvalTail(count, expr) => CommandJson::EvaluateTail {
                count,
                expr: ExprJson::from(expr),
            },
            Command::Query(id) => CommandJson::Query {
                identifier: id.as_ref().to_string(),
            },
            Command::Context => CommandJson::Context,
            Command::Unlambda(level, expr) => CommandJson::Unlambda {
                level,
                expr: ExprJson::from(expr),
            },
        }
    }
}
