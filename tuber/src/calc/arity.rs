use crate::calc::aliases::Aliases;
use crate::context::Context;
use crate::expr::Expr;

pub fn arity(context: &Context, aliases: &Aliases, expr: &Expr) -> Option<usize> {
    match expr {
        Expr::Lambda { .. } => Some(1),
        Expr::Variable(id) => {
            if aliases.has(id) {
                Some(0)
            } else {
                context.get(id).map(|f| f.arity())
            }
        }
        _ => None,
    }
}
