use crate::{expression::Expr, function::Func};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Command {
    Delete { identifier: String },
    Update { func: Func },
    Reduce { expr: Expr },
    ReduceLast { expr: Expr },
    ReduceHead { count: usize, expr: Expr },
    ReduceTail { count: usize, expr: Expr },
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
            tuber::Command::Eval(expr) => Command::Reduce {
                expr: Expr::from(expr),
            },
            tuber::Command::EvalLast(expr) => Command::ReduceLast {
                expr: Expr::from(expr),
            },
            tuber::Command::EvalHead(count, expr) => Command::ReduceHead {
                count,
                expr: Expr::from(expr),
            },
            tuber::Command::EvalTail(count, expr) => Command::ReduceTail {
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
            Command::Reduce { expr } => tuber::Command::Eval(expr.into()),
            Command::ReduceLast { expr } => tuber::Command::EvalLast(expr.into()),
            Command::ReduceHead { count, expr } => tuber::Command::EvalHead(count, expr.into()),
            Command::ReduceTail { count, expr } => tuber::Command::EvalTail(count, expr.into()),
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
