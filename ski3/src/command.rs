use crate::{expression::Expr, function::Func};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Command {
    Delete { identifier: String },
    Update { func: Func },
    Evaluate { expr: Expr },
    EvaluateLast { expr: Expr },
    EvaluateHead { count: usize, expr: Expr },
    EvaluateTail { count: usize, expr: Expr },
    Query { identifier: String },
    Context,
    Unlambda { level: u8, expr: Expr },
}

impl From<tuber::Command> for Command {
    fn from(tuber_command: tuber::Command) -> Command {
        match tuber_command {
            tuber::Command::Del(id) => Command::Delete {
                identifier: id.as_ref().to_string(),
            },
            tuber::Command::Update(func) => Command::Update {
                func: Func::from(func),
            },
            tuber::Command::Eval(expr) => Command::Evaluate {
                expr: Expr::from(expr),
            },
            tuber::Command::EvalLast(expr) => Command::EvaluateLast {
                expr: Expr::from(expr),
            },
            tuber::Command::EvalHead(count, expr) => Command::EvaluateHead {
                count,
                expr: Expr::from(expr),
            },
            tuber::Command::EvalTail(count, expr) => Command::EvaluateTail {
                count,
                expr: Expr::from(expr),
            },
            tuber::Command::Query(id) => Command::Query {
                identifier: id.as_ref().to_string(),
            },
            tuber::Command::Context => Command::Context,
            tuber::Command::Unlambda(level, expr) => Command::Unlambda {
                level,
                expr: Expr::from(expr),
            },
        }
    }
}

impl From<Command> for tuber::Command {
    fn from(ski_command: Command) -> tuber::Command {
        match ski_command {
            Command::Delete { identifier } => tuber::Command::Del(identifier.into()),
            Command::Update { func } => tuber::Command::Update(func.into()),
            Command::Evaluate { expr } => tuber::Command::Eval(expr.into()),
            Command::EvaluateLast { expr } => tuber::Command::EvalLast(expr.into()),
            Command::EvaluateHead { count, expr } => tuber::Command::EvalHead(count, expr.into()),
            Command::EvaluateTail { count, expr } => tuber::Command::EvalTail(count, expr.into()),
            Command::Query { identifier } => tuber::Command::Query(identifier.into()),
            Command::Context => tuber::Command::Context,
            Command::Unlambda { level, expr } => tuber::Command::Unlambda(level, expr.into()),
        }
    }
}

#[wasm_bindgen(js_name = parseCommand)]
pub fn parse_command(input: &str) -> Result<Command, JsError> {
    match tuber::parse_command(input) {
        Ok(command) => Ok(command.into()),
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}
