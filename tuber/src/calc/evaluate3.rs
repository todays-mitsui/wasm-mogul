use super::arity::arity;
use crate::context::Context;
use crate::expr::Expr;
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

    fn next_path(&self) -> Option<Vec<usize>> {
        let mut path = Vec::new();
        let mut inventory = &self.inventory;
        loop {
            match inventory.get_reducible() {
                ReducibleResult::Callee { .. } => {
                    return Some(path);
                }
                ReducibleResult::Arg { index, arg } => {
                    path.push(index);
                    inventory = arg;
                }
                ReducibleResult::None => {
                    return None;
                }
            }
        }
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
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
            redex: args
                .iter::<&Inventory>()
                .map(|arg| arg.reducible())
                .collect(),
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
#[derive(Debug, PartialEq)]
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
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    fn setup() -> Context {
        Context::default()
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
    fn test_eval_next_path() {
        let context = setup();

        let expr = expr::s("TRUE");
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), None);

        let expr = expr::v("TRUE");
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), None);

        let expr = expr::a(":i", ":x");
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), None);

        let expr = expr::a("i", ":x");
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), Some(vec![]));

        let expr = expr::a(expr::a("i", ":x"), ":y");
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), Some(vec![]));

        let expr = expr::a(":f", expr::a("i", ":x"));
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), Some(vec![0]));

        let expr = expr::a(expr::a("i", ":x"), expr::a("i", ":y"));
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), Some(vec![]));

        let expr = expr::a(expr::a(":i", ":x"), expr::a("i", ":y"));
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), Some(vec![1]));

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let eval = Eval::new(context.clone(), expr);
        assert_eq!(eval.next_path(), Some(vec![0, 0]));
    }
}
