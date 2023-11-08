use super::arity::arity;
use crate::context::Context;
use crate::expr::Expr;
use std::convert::identity;
use std::{cmp, iter, slice};

struct Eval {
    context: Context,
    inventory: Inventory,
}

impl Eval {
    fn new(context: Context, expr: Expr) -> Self {
        let inventory = Inventory::new(&context, expr);
        Self { context, inventory }
    }
}

// ========================================================================== //

struct Inventory {
    redex: Vec<bool>, // あとで独自型に書き換えるかも
    callee: Expr,
    args: Args,
}

impl Inventory {
    fn new(context: &Context, expr: Expr) -> Self {
        let mut callee = expr;
        let mut args = Args::new();

        while let Expr::Apply { lhs, rhs } = callee {
            callee = *lhs;
            args.unshift(context, *rhs);
        }

        let reducible: bool = arity(context, &callee)
            .map(|arity| args.len() >= cmp::max(1, arity))
            .unwrap_or_default();

        let mut redex = args.redex();
        redex.insert(0, reducible);

        Self {
            redex,
            callee,
            args,
        }
    }
}

enum BetaRedexResult {
    Callee { expr: Expr, arity: usize },
    Arg { index: usize, inventory: Inventory },
    Done,
}

// ========================================================================== //

/// ラムダ式の部分式のうち引数部分を保持する両端キュー
/// 実装の都合で内部的には引数を逆順で保持する
/// ```sxyz を分解して格納した場合、外部的には [x, y, z] として振る舞い、内部的には [z, y, x] というデータを保持する
struct Args(Vec<Inventory>);

impl Args {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn unshift(&mut self, context: &Context, expr: Expr) {
        let inventory = Inventory::new(context, expr);
        self.0.push(inventory)
    }

    // fn iter(&self) -> impl Iterator<Item = (usize, &Inventory)> {
    // TODO: このように impl Trait の形で書くとうまくいかない
    // TODO: Error: cannot move out of `args` because it is borrowed
    fn iter<Iter>(&self) -> iter::Enumerate<iter::Rev<slice::Iter<'_, Inventory>>> {
        self.0.iter().rev().enumerate()
    }

    fn redex(&self) -> Vec<bool> {
        self.0
            .iter()
            .map(|inventory| inventory.redex.iter().any(|bit| *bit))
            .collect()
    }
}

// ========================================================================== //
