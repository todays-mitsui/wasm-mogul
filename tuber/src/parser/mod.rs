mod ecmascript;
mod identifier;
mod lazy_k;
mod utils;

use crate::engine::Command;
use crate::expr::Expr;
use anyhow::Result;
pub use ecmascript::parse_command as parse_command_with_ecmascript_style;
pub use ecmascript::parse_expr as parse_expr_with_ecmascript_style;
pub use ecmascript::parse_update_or_delete as parse_update_or_delete_with_ecmascript_style;
pub use lazy_k::parse_command as parse_command_with_lazy_k_style;
pub use lazy_k::parse_expr as parse_expr_with_lazy_k_style;
pub use lazy_k::parse_update_or_delete as parse_update_or_delete_with_lazy_k_style;

pub fn parse_expr(input: &str) -> Result<Expr> {
    parse_expr_with_ecmascript_style(input).or_else(|_err| parse_expr_with_lazy_k_style(input))
}

pub fn parse_command(input: &str) -> Result<Command> {
    parse_command_with_ecmascript_style(input)
        .or_else(|_err| parse_command_with_lazy_k_style(input))
}

pub fn parse_update_or_delete(input: &str) -> Result<Command> {
    parse_update_or_delete_with_ecmascript_style(input)
        .or_else(|_err| parse_update_or_delete_with_lazy_k_style(input))
}
