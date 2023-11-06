use crate::context::Context;
use crate::expr::{self, Expr, Identifier};
use std::collections::HashSet;

pub fn expand(context: &Context, expr: Expr) -> Expr {
    let mut bound_vars = BoundVars::new();
    expand_(context, expr, &mut bound_vars)
}

fn expand_(context: &Context, expr: Expr, bound_vars: &mut BoundVars) -> Expr {
    match expr {
        Expr::Variable(ref id) if !bound_vars.contains(id) => match context.get(id) {
            Some(func) => {
                let mut bound_vars = BoundVars::new();
                expand_(context, func.to_owned().into(), &mut bound_vars)
            }
            None => expr,
        },
        Expr::Variable(_) => expr,
        Expr::Symbol(_) => expr,
        Expr::Apply { lhs, rhs } => expr::a(
            expand_(context, *lhs, bound_vars),
            expand_(context, *rhs, bound_vars),
        ),
        Expr::Lambda { param, body } => {
            bound_vars.insert(param.clone());
            expr::l(param, expand_(context, *body, bound_vars))
        }
    }
}

struct BoundVars(HashSet<Identifier>);

impl BoundVars {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert(&mut self, id: Identifier) {
        self.0.insert(id);
    }

    pub fn contains(&self, id: &Identifier) -> bool {
        self.0.contains(id)
    }
}
