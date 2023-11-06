mod command;
mod expression;

use crate::engine::Command;
use crate::expr::Expr;
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

pub fn parse_update_or_delete(s: &str) -> Result<Command> {
    let (command, rest) = update().easy_parse(s).map_err(|e| anyhow!("{}", e))?;

    if rest.is_empty() {
        match &command {
            Command::Update(_) => Ok(command),
            Command::Del(_) => Ok(command),
            _ => Err(anyhow!("unexpected command: {}", command)),
        }
    } else {
        Err(anyhow!("unexpected token: {}", rest))
    }
}
