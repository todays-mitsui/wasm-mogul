use super::apply::apply;
use super::arity::arity;
use crate::context::Context;
use crate::expr::{self, Expr};

#[derive(Debug, Clone, PartialEq)]
pub struct Eval<'a> {
    context: &'a Context,
    cursor: Cursor,
    current: Expr,
    stack: Stack<'a>,
}

/// 簡約のステップ
/// 最左最外簡約を行うために LeftTree → RightTree の順に簡約を試みる
/// 式全体を簡約し終えて正規形を得たら Done となる、それ以上簡約するべきものは何も無い
#[derive(Debug, Clone, PartialEq)]
enum Cursor {
    LeftTree,
    RightTree(usize),
    Done,
}

impl Eval<'_> {
    pub fn new(expr: Expr, context: &Context) -> Eval {
        Eval {
            context,
            cursor: Cursor::LeftTree,
            current: expr,
            stack: Stack::new(),
        }
    }

    // pub fn eval_last(&mut self, limit: usize) -> (Option<Expr>, bool) {
    //     assert!(0 < limit);

    //     if let Some(mut e) = self.next() {
    //         for _ in 0..limit - 1 {
    //             if let Some(next) = self.next() {
    //                 e = next;
    //             } else {
    //                 return (Some(e), false);
    //             }
    //         }

    //         // TODO: ここの true は嘘をつくことがある、peekable で先読みして正しい結果を返すように変える
    //         (Some(e), true)
    //     } else {
    //         (None, false)
    //     }
    // }

    fn expr(&self) -> Expr {
        let mut expr = self.current.clone();

        for arg in self.stack.all() {
            expr = expr::a(expr, arg.expr());
        }

        expr
    }
}

impl Iterator for Eval<'_> {
    type Item = EvalStep;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            Cursor::LeftTree => self.left_tree(),
            Cursor::RightTree(n) => self.right_tree(n),
            Cursor::Done => None,
        }
    }
}

impl Eval<'_> {
    fn left_tree(&mut self) -> Option<EvalStep> {
        // TODO: ここの clone 必要？
        while let Expr::Apply { lhs, rhs } = self.current.clone() {
            self.current = *lhs;
            self.stack.push(Eval::new(*rhs, self.context));
        }

        let maybe_args = arity(self.context, &self.current)
            .filter(|a| *a >= 1 || self.stack.len() >= 1)
            .and_then(|a| self.stack.pop(a));

        if let Some(args) = maybe_args {
            let result = apply(
                &self.context,
                &mut self.current,
                args.iter().map(|arg| arg.expr()).collect(),
            );
            assert!(result.is_ok());

            Some(EvalStep { expr: self.expr() })
        } else {
            self.cursor = Cursor::RightTree(0);

            self.next()
        }
    }

    fn right_tree(&mut self, n: usize) -> Option<EvalStep> {
        match self.stack.nth(n) {
            // スタックの n 番目の枝を取得し、その枝の簡約を試みる
            Some(step) => match step.next() {
                Some(_) => Some(EvalStep { expr: self.expr() }),

                // n 番目の枝が簡約済みなら、n+1 番目の枝へ進む
                None => {
                    self.cursor = Cursor::RightTree(n + 1);
                    self.next()
                }
            },

            // n がスタックの長さを超えているなら、もう簡約するべきものは何も無い
            None => {
                self.cursor = Cursor::Done;
                self.next()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EvalStep {
    pub expr: Expr,
    // ここに「次のステップでの簡約位置」などのメタ情報を持たせる想定
}

// ========================================================================== //

#[derive(Debug, Clone, PartialEq)]
struct Stack<'a>(Vec<Eval<'a>>);

impl<'a> Stack<'a> {
    fn new() -> Stack<'a> {
        Stack(Vec::new())
    }

    fn push(&mut self, expr: Eval<'a>) {
        self.0.push(expr);
    }

    fn pop(&mut self, n: usize) -> Option<Vec<Eval>> {
        let length = self.len();

        if length >= n {
            Some(self.0.drain(length - n..).rev().collect())
        } else {
            None
        }
    }

    fn all(&self) -> Vec<Eval> {
        let mut all = self.0.clone();
        all.reverse();
        all
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    /// 末尾から数えて n 番目の要素を取得する
    fn nth(&mut self, n: usize) -> Option<&mut Eval<'a>> {
        let len = self.0.len();
        if n >= len {
            None
        } else {
            self.0.get_mut(len - n - 1)
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_eval_steps_lambda_i() {
        let context = Context::new();

        let i = expr::l("x", "x");
        let expr = expr::a(i, ":a");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(steps.next().map(|step| step.expr), Some(expr::s("a")));
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_1() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(k, ":a");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(steps.next().map(|step| step.expr), Some(expr::l("y", ":a")));
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_2() {
        let context = Context::new();

        let k = expr::l("x", expr::l("y", "x"));
        let expr = expr::a(expr::a(k, ":a"), ":b");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(
            steps.next().map(|step| step.expr),
            Some(expr::a(expr::l("y", ":a"), ":b"))
        );
        assert_eq!(steps.next().map(|step| step.expr), Some(":a".into()));
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_true_1() {
        let context = setup();

        let expr = expr::v("TRUE");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_true_2() {
        let context = setup();

        let expr = expr::a(":a", "TRUE");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_true_3() {
        let context = setup();

        let expr = expr::a(expr::a("TRUE", ":a"), ":b");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(
            steps.next().map(|step| step.expr),
            Some(expr::a(expr::a(expr::a("k", "i"), ":a"), ":b"))
        );
        assert_eq!(steps.next().map(|step| step.expr), Some(expr::a("i", ":b")));
        assert_eq!(steps.next().map(|step| step.expr), Some(":b".into()));
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_i() {
        let context = setup();

        let expr = expr::a("i", ":a");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(steps.next().map(|step| step.expr), Some(":a".into()));
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_k_1() {
        let context = setup();

        let expr = expr::a("k", ":a");

        let mut steps = Eval::new(expr, &context);

        // k の arity が2なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_k_2() {
        let context = setup();

        let expr = expr::a(expr::a("k", ":a"), ":b");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(steps.next().map(|step| step.expr), Some(":a".into()));
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_s_1() {
        let context = setup();

        let expr = expr::a("s", ":a");

        let mut steps = Eval::new(expr, &context);

        // s の arity が3なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_s_2() {
        let context = setup();

        let expr = expr::a(expr::a("s", ":a"), ":b");

        let mut steps = Eval::new(expr, &context);

        // s の arity が3なのに対して引数を2つしか与えていないので簡約されない
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_func_s_3() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", ":a"), ":b"), ":c");

        let mut steps = Eval::new(expr, &context);

        assert_eq!(
            steps.next().map(|step| step.expr),
            Some(expr::a(expr::a(":a", ":c"), expr::a(":b", ":c")))
        );
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_skk() {
        let context = setup();

        let expr = expr::a(expr::a(expr::a("s", "k"), "k"), ":a");

        let steps = Eval::new(expr, &context);

        assert_eq!(steps.last().map(|step| step.expr), Some(":a".into()));
    }

    #[test]
    fn test_eval_steps_right_tree_1() {
        let context = setup();

        // `:a``k:b:c
        let expr = expr::a(expr::s("a"), expr::a(expr::a("k", ":b"), ":c"));

        let mut steps = Eval::new(expr, &context);

        assert_eq!(
            steps.next().map(|step| step.expr),
            Some(expr::a(":a", ":b"))
        );
        assert_eq!(steps.next().map(|step| step.expr), None);
    }

    #[test]
    fn test_eval_steps_right_tree_2() {
        let context = setup();

        // ```:a`i:b`i:c
        let expr = expr::a(expr::a(":a", expr::a("i", ":b")), expr::a("i", ":c"));

        let mut steps = Eval::new(expr, &context);

        assert_eq!(
            steps.next().map(|step| step.expr),
            Some(expr::a(expr::a(":a", ":b"), expr::a("i", ":c")))
        );
        assert_eq!(
            steps.next().map(|step| step.expr),
            Some(expr::a(expr::a(":a", ":b"), ":c"))
        );
        assert_eq!(steps.next().map(|step| step.expr), None);
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

        let mut steps = Eval::new(expr, &context);

        assert_eq!(
            steps.next().map(|step| step.expr),
            // ``^x.`x:a:c`^x.`x:b:c
            Some(expr::a(
                expr::a(expr::l("x", expr::a("x", ":a")), ":c"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
        );
        assert_eq!(
            steps.next().map(|step| step.expr),
            // ``:c:a`^x.`x:b:c
            Some(expr::a(
                expr::a(":c", ":a"),
                expr::a(expr::l("x", expr::a("x", ":b")), ":c")
            ))
        );
        assert_eq!(
            steps.next().map(|step| step.expr),
            // ``:c:a`:c:b
            Some(expr::a(expr::a(":c", ":a"), expr::a(":c", ":b")))
        );
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_stack_pop() {
        let context = Context::new();
        let mut stack = Stack(vec![
            Eval::new(expr::v("x"), &context),
            Eval::new(expr::v("y"), &context),
        ]);

        assert_eq!(stack.len(), 2);

        stack.push(Eval::new(expr::v("z"), &context));

        assert_eq!(stack.len(), 3);

        assert_eq!(
            stack.pop(2),
            Some(vec![
                Eval::new(expr::v("z"), &context),
                Eval::new(expr::v("y"), &context)
            ])
        );

        assert_eq!(stack.len(), 1);

        assert_eq!(stack.pop(1), Some(vec![Eval::new(expr::v("x"), &context)]));

        assert_eq!(stack.len(), 0);

        assert_eq!(stack.pop(1), None);
    }

    #[test]
    fn test_stack_all() {
        let context = Context::new();
        let stack = Stack(vec![
            Eval::new(expr::v("x"), &context),
            Eval::new(expr::v("y"), &context),
            Eval::new(expr::v("z"), &context),
        ]);
        assert_eq!(
            stack.all(),
            vec![
                Eval::new(expr::v("z"), &context),
                Eval::new(expr::v("y"), &context),
                Eval::new(expr::v("x"), &context),
            ]
        );

        let stack0 = Stack(vec![]);
        assert_eq!(stack0.all(), vec![]);
    }

    #[test]
    fn test_stack_nth() {
        let context = Context::new();
        let mut stack = Stack(vec![
            Eval::new(expr::v("x"), &context),
            Eval::new(expr::v("y"), &context),
            Eval::new(expr::v("z"), &context),
        ]);

        assert_eq!(stack.nth(0), Some(&mut Eval::new(expr::v("z"), &context)));
        assert_eq!(stack.nth(1), Some(&mut Eval::new(expr::v("y"), &context)));
        assert_eq!(stack.nth(2), Some(&mut Eval::new(expr::v("x"), &context)));
        assert_eq!(stack.nth(3), None);
    }

    // #[test]
    // fn test_eval_last_1() {
    //     let context = setup();

    //     let expr = ":a".into();
    //     let mut steps = Eval::new(expr, &context);

    //     assert_eq!(steps.eval_last(42), (None, false));
    // }

    // #[test]
    // fn test_eval_last_2() {
    //     let context = setup();

    //     let expr = expr::a("i", expr::a("i", expr::a("i", expr::a("i", ":a"))));
    //     let mut steps = Eval::new(expr, &context);

    //     assert_eq!(steps.eval_last(42), (Some(":a".into()), false));
    // }

    // #[test]
    // fn test_eval_last_3() {
    //     let context = setup();

    //     let expr = expr::a("i", expr::a("i", expr::a("i", expr::a("i", ":a"))));
    //     let mut steps = Eval::new(expr, &context);

    //     assert_eq!(steps.eval_last(3), (Some(expr::a("i", ":a")), true));
    // }
}
