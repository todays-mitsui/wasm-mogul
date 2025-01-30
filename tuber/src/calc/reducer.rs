use super::apply::apply;
use crate::calc;
use crate::context::Context;
use crate::expr::{self, Path, PathBuilder};

pub struct Reducer {
    context: Context,
    step: usize,
    expr: Expr,
    unary_reduction: bool,
}

pub struct ReduceResult {
    pub step: usize,
    pub expr: expr::Expr,
    pub reduced_path: Path,
}

impl Reducer {
    pub fn new(context: Context, expr: expr::Expr, unary_reduction: bool) -> Self {
        let expr = Expr::from(expr);
        Self {
            context,
            step: 0,
            expr,
            unary_reduction,
        }
    }

    pub fn expr(&self) -> expr::Expr {
        self.expr.clone().into()
    }

    pub fn reducible_path(&self) -> Option<Path> {
        self.expr
            .reducible_path(&self.context, self.unary_reduction)
    }
}

impl Iterator for Reducer {
    type Item = ReduceResult;

    fn next(&mut self) -> Option<Self::Item> {
        let reducible_path = self
            .expr
            .reducible_path(&self.context, self.unary_reduction)?;

        let reduced_path = self.expr.reduce(&self.context, &reducible_path);
        self.step += 1;
        self.unary_reduction = false;

        Some(ReduceResult {
            step: self.step,
            expr: expr::Expr::from(self.expr.to_owned()),
            reduced_path,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Expr {
    callee: expr::Expr,
    args: Vec<Expr>,
}

impl Expr {
    fn arity(&self, context: &Context) -> Option<usize> {
        calc::arity(context, &self.callee)
    }

    // self.callee に self.args のうちいくつかの項を与えて簡約可能かどうかを判定する
    fn callable(&self, context: &Context, unary_reduction: bool) -> bool {
        // unary_reduction == false の場合、
        // self.callee が arity = 0 の関数であっても、引数を伴わない単項の式なら簡約したくない
        // そのため簡約可能であるためには少なくとも1つ以上の引数を持つべきだ
        if !unary_reduction && self.args.len() == 0 {
            return false;
        }

        if let Some(arity) = self.arity(context) {
            // 簡約可能であるためには args が arity より長くなければいけない
            arity <= self.args.len()
        } else {
            // arity が取れないのは self.callee が未定義の関数である場合
            // そのときは簡約可能ではない
            false
        }
    }

    // self.callee に限らず self.args も再帰的にたどって簡約基を含むかどうかを判定する
    fn reducible(&self, context: &Context, unary_reduction: bool) -> bool {
        // TODO: reducible() が呼び出されるたびに args を再帰的に辿っていくのが非効率かもしれない、計算結果をキャッシュする機構を考えたい
        self.callable(context, unary_reduction)
            || self.args.iter().any(|arg| arg.reducible(context, false))
    }

    // 簡約基に至る経路を返す
    fn reducible_path(&self, context: &Context, unary_reduction: bool) -> Option<Path> {
        let mut path = PathBuilder::new();
        let mut expr = self;
        let mut unary_reduction = unary_reduction;

        loop {
            if expr.callable(context, unary_reduction) {
                path.set_arity(expr.arity(context).unwrap());
                return Some(path.build());
            } else {
                unary_reduction = false;
                match expr
                    .args
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, arg)| arg.reducible(context, false))
                {
                    Some((index, arg)) => {
                        path.add_route(index + 1);
                        expr = arg;
                    }
                    None => return None,
                }
            }
        }
    }

    // 簡約基を可変借用する
    fn reducible_expr(&mut self, reducible_path: &Path) -> &mut Expr {
        match reducible_path {
            Path::Arg(index, next) => {
                let rev_index = self.args.len() - index;
                self.args[rev_index].reducible_expr(next)
            }
            Path::Callee(_) => self,
        }
    }

    fn reduce(&mut self, context: &Context, reducible_path: &Path) -> Path {
        let expr = self.reducible_expr(reducible_path);
        let arity = expr.arity(context).unwrap();
        let args: Vec<expr::Expr> = expr
            .args
            .drain(expr.args.len() - arity..)
            .rev()
            .map(|expr| expr.into())
            .collect();
        let mut callee = &mut expr.callee;

        // TODO: エラー握りつぶしてるけど大丈夫？
        // TODO: apply() を reducer::Expr ベースに書き換えたい
        let _ = apply(context, callee, args);

        let mut num_args = 0;
        while let expr::Expr::Apply { lhs, rhs } = callee {
            num_args += 1;
            callee = lhs;
            expr.args.push(Expr::from(*rhs.to_owned()));
        }
        expr.callee = callee.to_owned();

        let mut reduced_path = reducible_path.clone();
        reduced_path.set_arity(num_args);
        return reduced_path;
    }
}

impl From<expr::Expr> for Expr {
    fn from(expr: expr::Expr) -> Expr {
        let mut callee: expr::Expr = expr;
        let mut args: Vec<Expr> = Vec::new();

        while let expr::Expr::Apply { lhs, rhs } = callee {
            callee = *lhs;
            args.push(Expr::from(*rhs));
        }

        Self { callee, args }
    }
}

impl From<Expr> for expr::Expr {
    fn from(expr: Expr) -> expr::Expr {
        let mut e = expr.callee;
        for arg in expr.args.into_iter().rev() {
            e = expr::a(e, expr::Expr::from(arg));
        }
        e
    }
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

        let _true = func::new("TRUE", Vec::<&str>::new(), "k");
        let _false = func::new("FALSE", Vec::<&str>::new(), expr::a("k", "i"));

        Context::from(vec![i, k, s, _true, _false])
    }

    #[test]
    fn test_reducer_new() {
        let context = setup();

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let reducer = Reducer::new(context.clone(), expr, false);

        assert_eq!(
            reducer.expr,
            Expr {
                callee: expr::s("g"),
                args: vec![Expr {
                    callee: expr::s("f"),
                    args: vec![Expr {
                        callee: expr::v("i"),
                        args: vec![Expr {
                            callee: expr::s("y"),
                            args: Vec::new(),
                        }],
                    }],
                }],
            }
        );
    }

    #[test]
    fn test_reducible_path() {
        let context = setup();

        let expr = expr::s("TRUE");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            None
        );

        let expr = expr::s("TRUE");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            None
        );

        let expr = expr::v("TRUE");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            None
        );

        let expr = expr::v("TRUE");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![0])
        );

        let expr = expr::a(":i", ":x");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            None
        );

        let expr = expr::a(":i", ":x");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            None
        );

        let expr = expr::a("i", ":x");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            Some(vec![1])
        );

        let expr = expr::a("i", ":x");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![1])
        );

        let expr = expr::a(":x", "i");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            None
        );

        let expr = expr::a(":x", "i");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            None
        );

        let expr = expr::a(expr::a("i", ":x"), ":y");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            Some(vec![1])
        );

        let expr = expr::a(expr::a("i", ":x"), ":y");
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![1])
        );

        let expr = expr::a(":f", expr::a("i", ":x"));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            Some(vec![1, 1])
        );

        let expr = expr::a(":f", expr::a("i", ":x"));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![1, 1])
        );

        let expr = expr::a(expr::a("i", ":x"), expr::a("i", ":y"));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            Some(vec![1])
        );

        let expr = expr::a(expr::a("i", ":x"), expr::a("i", ":y"));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![1])
        );

        let expr = expr::a(expr::a(":i", ":x"), expr::a("i", ":y"));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            Some(vec![2, 1])
        );

        let expr = expr::a(expr::a(":i", ":x"), expr::a("i", ":y"));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![2, 1])
        );

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, false).map(Vec::<usize>::from),
            Some(vec![1, 1, 1])
        );

        let expr = expr::a(":g", expr::a(":f", expr::a("i", ":y")));
        let expr = Expr::from(expr);
        assert_eq!(
            expr.reducible_path(&context, true).map(Vec::<usize>::from),
            Some(vec![1, 1, 1])
        );
    }

    #[test]
    fn test_expr_from_1() {
        // ```:a:b:c:d
        let e0: expr::Expr = expr::a(expr::a(expr::a(":a", ":b"), ":c"), ":d");

        let e1: Expr = Expr::from(e0.clone());
        let e2: expr::Expr = e1.into();

        assert_eq!(e0, e2);
    }

    #[test]
    fn test_expr_from_2() {
        // ```:a`:b:c:d`:e:f
        let e0: expr::Expr = expr::a(
            expr::a(expr::a(":a", expr::a(":b", ":c")), ":d"),
            expr::a(":e", ":f"),
        );

        let e1: Expr = Expr::from(e0.clone());
        let e2: expr::Expr = e1.into();

        assert_eq!(e0, e2);
    }

    #[test]
    fn test_reduce_result_lambda_i() {
        let context = Context::new();

        let i = expr::l("x", "x");
        let expr = expr::a(i, ":a");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(reducer.next().map(|result| result.expr), Some(expr::s("a")));
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_lambda_k_1() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(k, ":a");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::l("y", ":a"))
        );
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_lambda_k_2() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(expr::a(k, ":a"), ":b");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::a(expr::l("y", ":a"), ":b"))
        );
        assert_eq!(reducer.next().map(|result| result.expr), Some(":a".into()));
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_true_1_1() {
        let context = setup();

        let expr = expr::v("TRUE");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_true_1_2() {
        let context = setup();

        let expr = expr::v("TRUE");

        let mut reducer = Reducer::new(context, expr, true);

        assert_eq!(reducer.next().map(|result| result.expr), Some(expr::v("k")));
    }

    #[test]
    fn test_reduce_result_func_true_2() {
        let context = setup();

        let expr = expr::a(":a", "TRUE");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_true_3() {
        let context = setup();

        let expr = expr::a(expr::a("TRUE", ":a"), ":b");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::a(expr::a("k", ":a"), ":b"))
        );
        assert_eq!(reducer.next().map(|result| result.expr), Some(":a".into()));
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_i() {
        let context = setup();

        let expr = expr::a("i", ":a");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(reducer.next().map(|result| result.expr), Some(":a".into()));
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_k_1() {
        let context = setup();

        let expr = expr::a("k", ":a");

        let mut reducer = Reducer::new(context, expr, false);

        // k の arity が2なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_k_2() {
        let context = setup();

        let expr = expr::a(expr::a("k", ":a"), ":b");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(reducer.next().map(|result| result.expr), Some(":a".into()));
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_s_1() {
        let context = setup();

        let expr = expr::a("s", ":a");

        let mut reducer = Reducer::new(context, expr, false);

        // s の arity が3なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_s_2() {
        let context = setup();

        let expr = expr::a(expr::a("s", ":a"), ":b");

        let mut reducer = Reducer::new(context, expr, false);

        // s の arity が3なのに対して引数を2つしか与えていないので簡約されない
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_func_s_3() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", ":a"), ":b"), ":c");

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::a(expr::a(":a", ":c"), expr::a(":b", ":c")))
        );

        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_skk() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let reducer = Reducer::new(context, expr, false);

        assert_eq!(reducer.last().map(|result| result.expr), Some(":a".into()));
    }

    #[test]
    fn test_reduce_result_right_tree_1() {
        let context = setup();

        // `:a``k:b:c
        let expr = expr::a(expr::s("a"), expr::a(expr::a("k", ":b"), ":c"));

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::a(":a", ":b"))
        );
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce_result_right_tree_2() {
        let context = setup();

        // ```:a`i:b`i:c
        let expr = expr::a(expr::a(":a", expr::a("i", ":b")), expr::a("i", ":c"));

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::a(expr::a(":a", ":b"), expr::a("i", ":c")))
        );
        assert_eq!(
            reducer.next().map(|result| result.expr),
            Some(expr::a(expr::a(":a", ":b"), ":c"))
        );
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_reduce() {
        let context = setup();

        // ```s^x.`x:a^x.`x:b:c
        let expr = expr::a(
            expr::a(
                expr::a("s", expr::l("x", expr::a("x", ":a"))),
                expr::l("x", expr::a("x", ":b")),
            ),
            ":c",
        );

        let mut reducer = Reducer::new(context, expr, false);

        assert_eq!(
            reducer.next().map(|result| result.expr),
            // ``^x.`x:a:c`^x.`x:b:c
            Some(expr::a(
                expr::a(expr::l("x", expr::a("x", ":a")), ":c"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
        );
        assert_eq!(
            reducer.next().map(|result| result.expr),
            // ``:c:a`^x.`x:b:c
            Some(expr::a(
                expr::a(":c", ":a"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
        );
        assert_eq!(
            reducer.next().map(|result| result.expr),
            // ``:c:a`:c:b
            Some(expr::a(expr::a(":c", ":a"), expr::a(":c", ":b")))
        );
        assert_eq!(reducer.next().map(|result| result.expr), None);
    }

    #[test]
    fn test_eval_next_path() {
        let context = setup();
        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let mut reducer = Reducer::new(context, expr, false);
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            Some(vec![3])
        );

        reducer.next();
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            Some(vec![2])
        );

        reducer.next();
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            None
        );
    }

    #[test]
    fn test_reducer_reducible_path() {
        let context = setup();
        let expr = expr::a(expr::a(expr::a("s", "i"), expr::a("k", ":b")), ":a");

        let mut reducer = Reducer::new(context, expr, false);
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            Some(vec![3])
        );

        reducer.next();
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            Some(vec![1])
        );

        reducer.next();
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            Some(vec![1, 2])
        );

        reducer.next();
        assert_eq!(
            reducer.reducible_path().as_ref().map(Vec::<usize>::from),
            None
        );
    }

    #[test]
    fn test_reducer_reduced_path_1() {
        let context = setup();
        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let mut reducer = Reducer::new(context, expr, false);

        let result = reducer.next().unwrap();
        assert_eq!(Vec::<usize>::from(&result.reduced_path), vec![2]);

        let result = reducer.next().unwrap();
        assert_eq!(Vec::<usize>::from(&result.reduced_path), vec![0]);
    }

    #[test]
    fn test_reducer_reduced_path_2() {
        let context = setup();
        let expr = expr::a(expr::a(expr::a("s", "i"), expr::a("k", ":b")), ":a");

        let mut reducer = Reducer::new(context, expr, false);

        let result = reducer.next().unwrap();
        assert_eq!(Vec::<usize>::from(&result.reduced_path), vec![2]);

        let result = reducer.next().unwrap();
        assert_eq!(Vec::<usize>::from(&result.reduced_path), vec![0]);

        let result = reducer.next().unwrap();
        assert_eq!(Vec::<usize>::from(&result.reduced_path), vec![1, 0]);
    }
}
