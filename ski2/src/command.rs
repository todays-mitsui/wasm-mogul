use crate::{expression::ExprJson, function::FuncJson};
use js_sys::Object;
use serde::Serialize;
use tuber::Command;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum CommandJson {
    Delete { identifier: String },
    Update { func: FuncJson },
    Evaluate { expr: ExprJson },
    EvaluateLast { expr: ExprJson },
    EvaluateHead { count: usize, expr: ExprJson },
    EvaluateTail { count: usize, expr: ExprJson },
    Query { identifier: String },
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

impl From<CommandJson> for Command {
    fn from(command_json: CommandJson) -> Command {
        match command_json {
            CommandJson::Delete { identifier } => Command::Del(identifier.into()),
            CommandJson::Update { func } => Command::Update(func.into()),
            CommandJson::Evaluate { expr } => Command::Eval(expr.into()),
            CommandJson::EvaluateLast { expr } => Command::EvalLast(expr.into()),
            CommandJson::EvaluateHead { count, expr } => Command::EvalHead(count, expr.into()),
            CommandJson::EvaluateTail { count, expr } => Command::EvalTail(count, expr.into()),
            CommandJson::Query { identifier } => Command::Query(identifier.into()),
            CommandJson::Context => Command::Context,
            CommandJson::Unlambda { level, expr } => Command::Unlambda(level, expr.into()),
        }
    }
}

#[wasm_bindgen(js_name = parseCommand)]
pub fn parse_command(input: &str) -> Result<JsValue, JsError> {
    match tuber::parse_command(input) {
        Ok(command) => {
            let command_json: CommandJson = command.into();
            Ok(serde_wasm_bindgen::to_value(&command_json).unwrap())
        }
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}
