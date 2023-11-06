use super::arity::arity;
use crate::context::Context;
use crate::expr::Expr;
use std::cmp;

pub struct Eval {
    context: Context,
    inventory: Inventory,
}

impl Eval {
    pub fn new(context: Context, expr: Expr) -> Self {
        Self {
            inventory: Inventory::new(&context, expr),
            context,
        }
    }
}

// ========================================================================== //

struct Inventory {
    focus: Focus,
    callee: Expr,
    args: Args,
}

impl Inventory {
    fn new(context: &Context, expr: Expr) -> Self {
        let focus = Focus::Callee;
        let mut callee = expr;
        let mut args = Args::new();

        while let Expr::Apply { lhs, rhs } = callee {
            callee = *lhs;
            args.push(context, *rhs);
        }

        let callable: bool = arity(context, &callee)
            .map(|arity| args.len() >= cmp::max(0, arity))
            .unwrap_or_default();

        if callable {
            Self {
                focus,
                callee,
                args,
            }
        } else {
            for (index, arg) in args.0.iter().rev().enumerate() {
                if arg.focus() == &Focus::Done {
                    continue;
                } else {
                    return Self {
                        focus: Focus::Arg(index),
                        callee,
                        args,
                    };
                }
            }

            Self {
                focus: Focus::Done,
                callee,
                args,
            }
        }
    }

    fn focus(&self) -> &Focus {
        &self.focus
    }
}

// ========================================================================== //

#[derive(PartialEq)]
enum Focus {
    Callee,
    Arg(usize),
    Done,
}

// ========================================================================== //

struct Args(Vec<Inventory>);

impl Args {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, context: &Context, expr: Expr) {
        let inventory = Inventory::new(context, expr);
        self.0.push(inventory)
    }
}

// ========================================================================== //
