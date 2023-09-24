mod expression;

use crate::expr::Expr;
use combine::EasyParser;
pub use expression::expr;

pub fn parse_expr(s: &str) -> Result<Expr, String> {
    let (expr, rest) = expr().easy_parse(s).map_err(|e| format!("{:#?}", e))?;

    if rest.is_empty() {
        Ok(expr)
    } else {
        Err(format!("unexpected token: {}", rest))
    }
}
