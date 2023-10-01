mod recursive_iota;
mod recursive_sk;
mod recursive_ski;
mod shallow_sk;
mod shallow_ski;

use crate::context::Context;
use crate::expr::{Expr, Identifier};
pub use recursive_iota::unlambda as unlambda_recursive_iota;
pub use recursive_sk::unlambda as unlambda_recursive_sk;
pub use recursive_ski::unlambda as unlambda_recursive_ski;
pub use shallow_sk::unlambda as unlambda_shallow_sk;
pub use shallow_ski::unlambda as unlambda_shallow_ski;

pub fn unlambda_shallow(expr: Expr) -> Expr {
    let ski = [&("s".into()), &("k".into()), &("i".into())];
    unlambda_shallow_ski(expr, &ski)
}

pub fn unlambda_shallow_(strategy: &ShallowStrategy, expr: Expr) -> Expr {
    match strategy {
        ShallowStrategy::SKI => {
            let ski = [&("s".into()), &("k".into()), &("i".into())];
            unlambda_shallow_ski(expr, &ski)
        }
        ShallowStrategy::SKIWith { s, k, i } => unlambda_shallow_ski(expr, &[s, k, i]),
        ShallowStrategy::SK => {
            let sk = [&("s".into()), &("k".into())];
            unlambda_shallow_sk(expr, &sk)
        }
        ShallowStrategy::SKWith { s, k } => unlambda_shallow_sk(expr, &[s, k]),
    }
}

pub fn unlambda_recursive(context: &Context, expr: Expr) -> Expr {
    let ski = [&("s".into()), &("k".into()), &("i".into())];
    unlambda_recursive_ski(context, expr, &ski)
}

pub fn unlambda_iota(context: &Context, expr: Expr) -> Expr {
    unlambda_recursive_iota(context, expr, &("ι".into()))
}

pub fn unlambda_recursive_(strategy: &RecursiveStrategy, context: &Context, expr: Expr) -> Expr {
    match strategy {
        RecursiveStrategy::SKI => {
            let ski = [&("s".into()), &("k".into()), &("i".into())];
            unlambda_recursive_ski(context, expr, &ski)
        }
        RecursiveStrategy::SKIWith { s, k, i } => unlambda_recursive_ski(context, expr, &[s, k, i]),
        RecursiveStrategy::SK => {
            let sk = [&("s".into()), &("k".into())];
            unlambda_recursive_sk(context, expr, &sk)
        }
        RecursiveStrategy::SKWith { s, k } => unlambda_recursive_sk(context, expr, &[s, k]),
        RecursiveStrategy::Iota => unlambda_recursive_iota(context, expr, &("ι".into())),
        RecursiveStrategy::IotaWith { iota } => unlambda_recursive_iota(context, expr, iota),
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
pub enum ShallowStrategy {
    SKI,
    SKIWith {
        s: Identifier,
        k: Identifier,
        i: Identifier,
    },
    SK,
    SKWith {
        s: Identifier,
        k: Identifier,
    },
}

#[derive(Debug, PartialEq)]
pub enum RecursiveStrategy {
    SKI,
    SKIWith {
        s: Identifier,
        k: Identifier,
        i: Identifier,
    },
    SK,
    SKWith {
        s: Identifier,
        k: Identifier,
    },
    Iota,
    IotaWith {
        iota: Identifier,
    },
}
