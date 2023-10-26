use crate::context::Context;
use crate::expr::Expr;

pub fn arity(context: &Context, expr: &Expr) -> Option<usize> {
    match expr {
        Expr::Lambda { .. } => Some(1),
        Expr::Variable(id) => context.get(id).map(|f| f.arity()),
        _ => None,
    }
}
