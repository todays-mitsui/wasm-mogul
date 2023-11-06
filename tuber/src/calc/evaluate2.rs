use crate::context::Context;
use crate::expr::Expr;

pub struct Eval {
    context: Context,
    inventory: Inventory,
}

struct Inventory {
    focus: Focus,
    callee: Expr,
    args: Args,
}

enum Focus {
    Callee,
    Arg(usize),
    Done,
}

struct Args(Vec<Inventory>);
