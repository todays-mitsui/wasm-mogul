use crate::calc::EvalStep;
use crate::context::Context;
use crate::expr::{Expr, Identifier};
use crate::func::Func;

pub enum Output {
    Del {
        input: Identifier,
        result: Context,
    },
    Update {
        input: Func,
        result: Context,
    },
    Eval {
        input: Expr,
        steps: Vec<EvalStep>,
    },
    Search {
        input: Identifier,
        result: Option<Func>,
    },
    Context {
        result: Context,
    },
    Unlambda {
        input: Expr,
        result: Expr,
    },
}
