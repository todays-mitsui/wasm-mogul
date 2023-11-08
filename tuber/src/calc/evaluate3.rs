use super::apply::apply;
use super::arity::arity;
use crate::context::Context;
use crate::expr::{self, Expr};
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

impl Iterator for Eval {
    type Item = EvalStep;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: なんか思ったよりも汚くなったのでリファクタリングが必要

        let inventory = self.inventory.get_next()?;

        let args = inventory.args.shift(inventory.arity?)?;
        let mut callee = &mut inventory.callee;

        apply(&self.context, callee, args).ok()?;

        while let Expr::Apply { lhs, rhs } = callee {
            callee = lhs;
            inventory.args.unshift(&self.context, *rhs.to_owned());
        }

        inventory.callee = callee.to_owned();
        inventory.arity = arity(&self.context, &inventory.callee)
            .filter(|arity| inventory.args.len() >= cmp::max(1, *arity));
        inventory.redex = inventory.args.iter().map(|arg| arg.reducible()).collect();

        Some(EvalStep {
            expr: self.inventory.clone().into(),
        })
    }
}

// ========================================================================== //

#[derive(Clone, Debug, PartialEq)]
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

        let arity = arity(context, &callee).filter(|arity| args.len() >= cmp::max(1, *arity));

        Self {
            callee,
            arity,
            redex: args.iter().map(|arg| arg.reducible()).collect(),
            args,
        }
    }

    fn reducible(&self) -> bool {
        self.arity.is_some() || self.redex.iter().any(|b| *b)
    }

    fn get_reducible(&mut self) -> ReducibleResult {
        match self.arity {
            Some(arity) => ReducibleResult::Callee {
                expr: &mut self.callee,
                arity,
            },
            None => match self
                .args
                .enumerate_mut::<(usize, &mut Inventory)>()
                .find(|(_index, arg)| arg.reducible())
            {
                Some((index, arg)) => ReducibleResult::Arg { index, arg },
                None => ReducibleResult::None,
            },
        }
    }

    fn next_path(&self) -> Option<Vec<usize>> {
        let mut path = Vec::new();
        let mut inventory = self;
        loop {
            if let Some(_) = inventory.arity {
                return Some(path);
            } else {
                match inventory
                    .args
                    .enumerate::<(usize, &Inventory)>()
                    .find(|(_index, arg)| arg.reducible())
                {
                    Some((index, arg)) => {
                        path.push(index);
                        inventory = arg;
                    }
                    None => return None,
                }
            }
        }
    }

    fn get_next(&mut self) -> Option<&mut Inventory> {
        let mut inventory = self;
        loop {
            if let Some(_) = inventory.arity {
                return Some(inventory);
            } else {
                match inventory
                    .args
                    .enumerate_mut::<(usize, &mut Inventory)>()
                    .find(|(_index, arg)| arg.reducible())
                {
                    Some((_index, arg)) => {
                        inventory = arg;
                    }
                    None => return None,
                }
            }
        }
    }
}

impl From<Inventory> for Expr {
    fn from(inventory: Inventory) -> Self {
        let mut expr = inventory.callee;
        for arg in inventory.args.0.into_iter().rev() {
            expr = expr::a(expr, Expr::from(arg));
        }
        expr
    }
}

enum ReducibleResult<'a> {
    Callee {
        expr: &'a mut Expr,
        arity: usize,
    },
    Arg {
        index: usize,
        arg: &'a mut Inventory,
    },
    None,
}

// ========================================================================== //

/// ラムダ式の部分式のうち引数部分を保持する両端キュー
/// 実装の都合で内部的には引数を逆順で保持する
/// ```sxyz を分解して格納した場合、外部的には [x, y, z] として振る舞い、内部的には [z, y, x] というデータを保持する
#[derive(Clone, Debug, PartialEq)]
struct Args(Vec<Inventory>);

impl Args {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn shift(&mut self, n: usize) -> Option<Vec<Expr>> {
        let length = self.len();

        if length >= n {
            Some(
                self.0
                    .drain(length - n..)
                    .rev()
                    .map(|inventory| inventory.into())
                    .collect(),
            )
        } else {
            None
        }
    }

    fn unshift(&mut self, context: &Context, expr: Expr) {
        let inventory = Inventory::new(context, expr);
        self.0.push(inventory)
    }

    fn join(&mut self, other: Self) {
        self.0.extend(other.0.into_iter().rev())
    }

    fn iter(&self) -> ArgsIter {
        ArgsIter::new(self.to_owned())
    }

    fn enumerate<Iter>(&self) -> iter::Enumerate<iter::Rev<slice::Iter<'_, Inventory>>> {
        self.0.iter().rev().enumerate()
    }

    fn enumerate_mut<Iter>(&mut self) -> iter::Enumerate<iter::Rev<slice::IterMut<'_, Inventory>>> {
        self.0.iter_mut().rev().enumerate()
    }

    // // fn iter(&self) -> impl Iterator<Item = (usize, &Inventory)> {
    // // TODO: このように impl Trait の形で書くとうまくいかない
    // // TODO: Error: cannot move out of `args` because it is borrowed
    // fn iter<Iter>(&self) -> iter::Rev<slice::Iter<'_, Inventory>> {
    //     self.0.iter().rev()
    // }
}

impl From<Vec<Inventory>> for Args {
    fn from(args: Vec<Inventory>) -> Self {
        Self(args)
    }
}

struct ArgsIter {
    iter: std::iter::Rev<std::vec::IntoIter<Inventory>>,
}

impl ArgsIter {
    fn new(args: Args) -> Self {
        Self {
            iter: args.0.into_iter().rev(),
        }
    }
}

impl Iterator for ArgsIter {
    type Item = Inventory;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
pub struct EvalStep {
    pub expr: Expr,
    // ここに「次のステップでの簡約位置」などのメタ情報を持たせる想定
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::func;

    fn setup() -> Context {
        let i = func::new("i", vec!["x"], "x");
        let k = func::new("k", vec!["x", "y"], "x");
        let s = func::new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        );

        let _true = func::new("TRUE", Vec::<&str>::new(), expr::a("k", "i"));
        let _false = func::new("FALSE", Vec::<&str>::new(), "k");

        Context::from(vec![i, k, s, _true, _false])
    }

    #[test]
    fn test_eval_new() {
        let context = setup();

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let eval = Eval::new(context.clone(), expr);

        assert_eq!(
            eval.inventory,
            Inventory {
                callee: expr::s("g"),
                arity: None,
                args: Args(vec![Inventory {
                    callee: expr::s("f"),
                    arity: None,
                    args: Args(vec![Inventory {
                        callee: expr::v("i"),
                        arity: Some(1),
                        args: Args(vec![Inventory {
                            callee: expr::s("y"),
                            arity: None,
                            args: Args::new(),
                            redex: vec![],
                        },]),
                        redex: vec![false],
                    }]),
                    redex: vec![true],
                },]),
                redex: vec![true],
            }
        );
    }

    #[test]
    fn test_inventory_next_path() {
        let context = setup();

        let expr = expr::s("TRUE");
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), None);

        let expr = expr::v("TRUE");
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), None);

        let expr = expr::a(":i", ":x");
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), None);

        let expr = expr::a("i", ":x");
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(vec![]));

        let expr = expr::a(expr::a("i", ":x"), ":y");
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(vec![]));

        let expr = expr::a(":f", expr::a("i", ":x"));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(vec![0]));

        let expr = expr::a(expr::a("i", ":x"), expr::a("i", ":y"));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(vec![]));

        let expr = expr::a(expr::a(":i", ":x"), expr::a("i", ":y"));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(vec![1]));

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(vec![0, 0]));
    }

    #[test]
    fn test_eval_steps_lambda_i() {
        let context = Context::new();

        let i = expr::l("x", "x");
        let expr = expr::a(i, ":a");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), Some(expr::s("a")));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_1() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(k, ":a");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), Some(expr::l("y", ":a")));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_2() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(expr::a(k, ":a"), ":b");

        let mut eval = Eval::new(context, expr);

        assert_eq!(
            eval.next().map(|step| step.expr),
            Some(expr::a(expr::l("y", ":a"), ":b"))
        );
        assert_eq!(eval.next().map(|step| step.expr), Some(":a".into()));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_true_1() {
        let context = setup();

        let expr = expr::v("TRUE");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_true_2() {
        let context = setup();

        let expr = expr::a(":a", "TRUE");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_true_3() {
        let context = setup();

        let expr = expr::a(expr::a("TRUE", ":a"), ":b");

        let mut eval = Eval::new(context, expr);

        assert_eq!(
            eval.next().map(|step| step.expr),
            Some(expr::a(expr::a(expr::a("k", "i"), ":a"), ":b"))
        );
        assert_eq!(eval.next().map(|step| step.expr), Some(expr::a("i", ":b")));
        assert_eq!(eval.next().map(|step| step.expr), Some(":b".into()));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }
}
