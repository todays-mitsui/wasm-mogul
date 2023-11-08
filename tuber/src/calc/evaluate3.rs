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
    callee: Expr,
    arity: Option<usize>,
    args: Args,
    redex: Vec<bool>, // あとで独自型に書き換えるかも
}

impl Inventory {
    fn new(context: &Context, expr: Expr) -> Self {
        let mut callee = expr;
        let mut args = Args::new();

        while let Expr::Apply { lhs, rhs } = callee {
            callee = *lhs;
            args.unshift(context, *rhs);
        }

        let maybe_arity = arity(context, &callee).filter(|arity| args.len() >= cmp::max(1, *arity));

        Self {
            callee,
            arity: maybe_arity,
            redex: args.redex(),
            args,
        }
    }

    fn reducible(&self) -> bool {
        self.arity.is_some() || self.redex.iter().any(|b| *b)
    }

    fn get_reducible(&self) -> ReducibleResult {
        match self.arity {
            Some(arity) => ReducibleResult::Callee {
                expr: &self.callee,
                arity,
            },
            None => match self
                .args
                .enumerate::<(usize, &Inventory)>()
                .find(|(_index, arg)| arg.reducible())
            {
                Some((index, arg)) => ReducibleResult::Arg { index, arg },
                None => ReducibleResult::None,
            },
        }
    }
}

enum ReducibleResult<'a> {
    Callee { expr: &'a Expr, arity: usize },
    Arg { index: usize, arg: &'a Inventory },
    None,
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

    fn enumerate<Iter>(&self) -> iter::Enumerate<iter::Rev<slice::Iter<'_, Inventory>>> {
        self.0.iter().rev().enumerate()
    }

    // fn iter(&self) -> impl Iterator<Item = (usize, &Inventory)> {
    // TODO: このように impl Trait の形で書くとうまくいかない
    // TODO: Error: cannot move out of `args` because it is borrowed
    fn iter<Iter>(&self) -> iter::Rev<slice::Iter<'_, Inventory>> {
        self.0.iter().rev()
    }

    /// 各 arg が簡約可能な部分式を含むかどうかを調べる
    fn redex(&self) -> Vec<bool> {
        self.0
            .iter()
            .map(|inventory| {
                inventory
                    .args
                    .iter::<&Inventory>()
                    .map(|inner| inner.arity.is_some())
                    .any(|b| b)
            })
            .collect()
    }
}

// ========================================================================== //
