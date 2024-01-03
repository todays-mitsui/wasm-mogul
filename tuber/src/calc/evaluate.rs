use super::apply::apply;
use super::arity::arity;
use super::path::{Path, PathBuilder};
use crate::context::Context;
use crate::expr::{self, Expr};
use std::{cmp, iter, slice};

#[derive(Clone, Debug, PartialEq)]
pub struct Eval {
    context: Context,
    next_path: Option<Path>,
    inventory: Inventory,
    step: usize,
}

impl Eval {
    pub fn new(context: Context, expr: Expr) -> Self {
        let inventory = Inventory::new(&context, expr);
        Self {
            context,
            next_path: inventory.next_path(),
            inventory,
            step: 0,
        }
    }
}

impl Iterator for Eval {
    type Item = EvalStep;

    fn next(&mut self) -> Option<Self::Item> {
        let inventory = self.inventory.get_next()?;

        match inventory.eval(&self.context) {
            Some(()) => {
                let inventory = self.inventory.clone();
                let next_path = inventory.next_path();
                let expr = inventory.into();
                self.step += 1;
                Some(EvalStep {
                    expr,
                    step: self.step,
                    next_path,
                })
            }
            None => None,
        }
    }
}

// ========================================================================== //

#[derive(Clone, Debug, PartialEq)]
struct Inventory {
    callee: Expr,
    arity: Option<usize>,
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

        let arity = arity(context, &callee).filter(|arity| args.len() >= cmp::max(1, *arity));

        Self {
            callee,
            arity,
            args,
        }
    }

    fn reducible(&self) -> bool {
        let reducible_callee = || self.arity.is_some();
        let reducible_args = || self.args.iter().any(|arg| arg.reducible()); // TODO: reducible() が呼び出されるたびに args.iter() を再帰的に辿っていくのが非効率かもしれない、計算結果をキャッシュする機構を考えたい
        reducible_callee() || reducible_args()
    }

    fn next_path(&self) -> Option<Path> {
        let mut builder = PathBuilder::new();
        let mut inventory = self;
        loop {
            if let Some(arity) = inventory.arity {
                builder.set_arity(arity);
                let path = builder.build();
                return Some(path);
            } else {
                match inventory
                    .args
                    .enumerate::<(usize, &Inventory)>()
                    .find(|(_index, arg)| arg.reducible())
                {
                    Some((index, arg)) => {
                        builder.add_route(index);
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

    fn eval(&mut self, context: &Context) -> Option<()> {
        let args = self.args.drain(self.arity?)?;
        let mut callee = &mut self.callee;

        apply(context, callee, args).ok()?;

        while let Expr::Apply { lhs, rhs } = callee {
            callee = lhs;
            self.args.unshift(context, *rhs.to_owned());
        }

        self.callee = callee.to_owned();
        self.arity =
            arity(context, &self.callee).filter(|arity| self.args.len() >= cmp::max(1, *arity));

        Some(())
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

    fn unshift(&mut self, context: &Context, expr: Expr) {
        let inventory = Inventory::new(context, expr);
        self.0.push(inventory)
    }

    fn drain(&mut self, n: usize) -> Option<Vec<Expr>> {
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

    // TODO: ここから下もっとどうにかしたい
    fn iter(&self) -> ArgsIter {
        ArgsIter::new(self.to_owned())
    }

    fn enumerate<Iter>(&self) -> iter::Enumerate<iter::Rev<slice::Iter<'_, Inventory>>> {
        self.0.iter().rev().enumerate()
    }

    fn enumerate_mut<Iter>(&mut self) -> iter::Enumerate<iter::Rev<slice::IterMut<'_, Inventory>>> {
        self.0.iter_mut().rev().enumerate()
    }
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
    pub step: usize,
    pub expr: Expr,
    pub next_path: Option<Path>,
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
                        },]),
                    }]),
                },]),
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
        assert_eq!(inventory.next_path(), Some(Path::new(vec![], 1)));

        let expr = expr::a(expr::a("i", ":x"), ":y");
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(Path::new(vec![], 1)));

        let expr = expr::a(":f", expr::a("i", ":x"));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(Path::new(vec![0], 1)));

        let expr = expr::a(expr::a("i", ":x"), expr::a("i", ":y"));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(Path::new(vec![], 1)));

        let expr = expr::a(expr::a(":i", ":x"), expr::a("i", ":y"));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(Path::new(vec![1], 1)));

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let inventory = Inventory::new(&context, expr);
        assert_eq!(inventory.next_path(), Some(Path::new(vec![0, 0], 1)));
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

    #[test]
    fn test_eval_steps_func_i() {
        let context = setup();

        let expr = expr::a("i", ":a");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), Some(":a".into()));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_k_1() {
        let context = setup();

        let expr = expr::a("k", ":a");

        let mut eval = Eval::new(context, expr);

        // k の arity が2なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_k_2() {
        let context = setup();

        let expr = expr::a(expr::a("k", ":a"), ":b");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), Some(":a".into()));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_s_1() {
        let context = setup();

        let expr = expr::a("s", ":a");

        let mut eval = Eval::new(context, expr);

        // s の arity が3なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_s_2() {
        let context = setup();

        let expr = expr::a(expr::a("s", ":a"), ":b");

        let mut eval = Eval::new(context, expr);

        // s の arity が3なのに対して引数を2つしか与えていないので簡約されない
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_s_3() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", ":a"), ":b"), ":c");

        let mut eval = Eval::new(context, expr);

        assert_eq!(
            eval.next().map(|step| step.expr),
            Some(expr::a(expr::a(":a", ":c"), expr::a(":b", ":c")))
        );
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_skk() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.last().map(|step| step.expr), Some(":a".into()));
    }

    #[test]
    fn test_eval_steps_right_tree_1() {
        let context = setup();

        // `:a``k:b:c
        let expr = expr::a(expr::s("a"), expr::a(expr::a("k", ":b"), ":c"));

        let mut eval = Eval::new(context, expr);

        assert_eq!(eval.next().map(|step| step.expr), Some(expr::a(":a", ":b")));
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_right_tree_2() {
        let context = setup();

        // ```:a`i:b`i:c
        let expr = expr::a(expr::a(":a", expr::a("i", ":b")), expr::a("i", ":c"));

        let mut eval = Eval::new(context, expr);

        assert_eq!(
            eval.next().map(|step| step.expr),
            Some(expr::a(expr::a(":a", ":b"), expr::a("i", ":c")))
        );
        assert_eq!(
            eval.next().map(|step| step.expr),
            Some(expr::a(expr::a(":a", ":b"), ":c"))
        );
        assert_eq!(eval.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps() {
        let context = setup();

        // ```s^x.`x:a^x.`x:b:c
        let expr = expr::a(
            expr::a(
                expr::a("s", expr::l("x", expr::a("x", ":a"))),
                expr::l("x", expr::a("x", ":b")),
            ),
            ":c",
        );

        let mut eval = Eval::new(context, expr);

        assert_eq!(
            eval.next().map(|step| step.expr),
            // ``^x.`x:a:c`^x.`x:b:c
            Some(expr::a(
                expr::a(expr::l("x", expr::a("x", ":a")), ":c"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
        );
        assert_eq!(
            eval.next().map(|step| step.expr),
            // ``:c:a`^x.`x:b:c
            Some(expr::a(
                expr::a(":c", ":a"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
        );
        assert_eq!(
            eval.next().map(|step| step.expr),
            // ``:c:a`:c:b
            Some(expr::a(expr::a(":c", ":a"), expr::a(":c", ":b")))
        );
        assert_eq!(eval.next(), None);
    }

    #[test]
    fn test_eval_next_path() {
        let context = setup();
        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let mut eval = Eval::new(context, expr);
        assert_eq!(eval.next_path, Some(Path::new(vec![], 3)));

        let step = eval.next().unwrap();
        assert_eq!(step.next_path, Some(Path::new(vec![], 2)));

        let step = eval.next().unwrap();
        assert_eq!(step.next_path, None);
    }

    #[test]
    fn test_eval_next_path_2() {
        let context = setup();
        let expr = expr::a(expr::a(expr::a("s", "i"), expr::a("k", ":b")), ":a");

        let mut eval = Eval::new(context, expr);
        assert_eq!(eval.next_path, Some(Path::new(vec![], 3)));

        let step = eval.next().unwrap();
        assert_eq!(step.next_path, Some(Path::new(vec![], 1)));

        let step = eval.next().unwrap();
        assert_eq!(step.next_path, Some(Path::new(vec![0], 2)));

        let step = eval.next().unwrap();
        assert_eq!(step.next_path, None);
    }
}
