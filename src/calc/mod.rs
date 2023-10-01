mod apply;
mod arity;
mod evaluate;
mod unlambda;

use crate::context::Context;
use crate::expr::Expr;
use evaluate::EvalSteps;
pub use unlambda::{
    unlambda_iota, unlambda_recursive, unlambda_recursive_, unlambda_shallow, unlambda_shallow_,
    RecursiveStrategy, ShallowStrategy,
};

pub fn eval_steps(context: &Context, expr: Expr) -> EvalSteps {
    EvalSteps::new(expr, context)
}
