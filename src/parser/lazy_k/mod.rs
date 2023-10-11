mod command;
mod expression;

use crate::engine::Command;
use crate::expr::Expr;
use crate::func::Func;
use anyhow::{anyhow, Result};
use combine::EasyParser;
pub use command::{command, update};
pub use expression::expr;

pub fn parse_expr(s: &str) -> Result<Expr> {
    let (expr, rest) = expr().easy_parse(s).map_err(|e| anyhow!("{}", e))?;

    if rest.is_empty() {
        Ok(expr)
    } else {
        Err(anyhow!("unexpected token: {}", rest))
    }
}

pub fn parse_command(s: &str) -> Result<Command> {
    let (command, rest) = command().easy_parse(s).map_err(|e| anyhow!("{}", e))?;

    if rest.is_empty() {
        Ok(command)
    } else {
        Err(anyhow!("unexpected token: {}", rest))
    }
}

pub fn parse_func(s: &str) -> Result<Func> {
    let (command, rest) = update().easy_parse(s).map_err(|e| anyhow!("{}", e))?;

    if rest.is_empty() {
        if let Command::Update(func) = command {
            Ok(func)
        } else {
            Err(anyhow!("unexpected command: {}", command))
        }
    } else {
        Err(anyhow!("unexpected token: {}", rest))
    }
}
