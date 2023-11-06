use super::apply::apply;
use super::arity::arity;
use crate::context::Context;
use crate::expr::Expr;

pub struct Eval {
    context: Context,
    focus: Focus,
    expr: Expr,
}

impl Eval {
    pub fn new(expr: Expr, context: &Context) -> Self {
        let focus = Focus::Route(vec![]);
        let expr = expr;

        Self {
            context: context.clone(),
            focus,
            expr,
        }
    }

    fn focus(&mut self) {}

    pub fn next(&mut self) -> Option<()> {
        match &self.focus {
            Focus::Done => None,

            Focus::Route(route) if route.is_empty() => {
                let mut args = Args::new();
                let mut expr = &mut self.expr;

                while let Expr::Apply { lhs, rhs } = expr {
                    expr = lhs;
                    args.push(rhs);
                }

                let maybe_args = arity(&self.context, &expr)
                    .filter(|arity| *arity >= 1 || args.len() >= 1)
                    .and_then(|arity| args.pop(arity));

                if let Some(args) = maybe_args {
                    // β簡約に必要なだけの引数があれば、簡約を行い結果を返す
                    let result = apply(
                        &self.context,
                        expr,
                        args.into_iter().map(|expr| expr.to_owned()).collect(),
                    );
                    assert!(result.is_ok());

                    Some(())
                } else {
                    // β簡約に必要な引数が無ければ、右側の枝の簡約を試みる
                    self.focus = Focus::Route(vec![0]);
                    self.next()
                }
            }

            Focus::Route(route) => {
                let index = route.get(route.len() - 1).unwrap();

                let mut args = Args::new();
                let mut expr = &mut self.expr;

                while let Expr::Apply { lhs, rhs } = expr {
                    expr = lhs;
                    args.push(rhs);
                }

                let arg = args.nth(*index);

                match arg {
                    Some(arg) => {
                        let focus = Focus::Route(route.clone());
                        self.expr = arg.to_owned();
                        Some(())
                    }

                    None => {
                        self.focus = Focus::Route(route.clone());
                        self.next()
                    }
                }
            }
        }
    }
}

/// foucus を辿って expr の部分式を返す
fn walk_to(mut focus: Focus, expr: &mut Expr) -> &mut Expr {
    if let Focus::Done = focus {
        // すでに簡約済みなら式全体をそのまま返す
        return expr;
    }

    let mut expr = expr;
    let mut args = Args::new();

    while let Expr::Apply { lhs, rhs } = expr {
        expr = lhs;
        args.push(rhs);
    }

    match focus.shift() {
        Some(index) => match args.nth(index) {
            Some(arg) => walk_to(focus, arg),
            None => unreachable!("存在しない道を進もうとしてない？"),
        },
        None => expr,
    }
}

fn update_focus(context: &Context, expr: &mut Expr, focus: &mut Focus) {
    let expr = walk_to(focus.clone(), expr);
    update_focus_(context, expr, focus)
}

fn update_focus_(context: &Context, expr: &Expr, focus: &mut Focus) {
    if let Focus::Done = focus {
        // すでに簡約済みなら何もすることは無い
        return;
    }

    let mut args: Vec<&Expr> = Vec::new();
    let mut expr = expr;

    while let Expr::Apply { lhs, rhs } = expr {
        expr = lhs;
        args.push(rhs);
    }

    if args.is_empty() {
        // 引数が無ければ簡約済みとみなす
        *focus = Focus::Done;
        return;
    }

    let can_substitute = arity(context, expr)
        .map(|arity| arity >= 1 || args.len() >= arity)
        .unwrap_or(false);

    if can_substitute {
        // 左部分式が簡約対象なら focus を変更する必要はない、そのまま return する
        return;
    }

    for (index, ref mut arg) in args.iter().enumerate() {
        let new_focus = focus.push(index);
        /* Done かどうかチェックが必要 */
        update_focus(context, arg, focus);
    }
}

// ========================================================================== //

#[derive(Clone, Debug, PartialEq)]
enum Focus {
    Done,
    Route(Vec<usize>),
}

impl Focus {
    fn push(&self, index: usize) -> Self {
        assert_ne!(
            self,
            &Focus::Done,
            "すでに簡約が完了しているため Focus を移動できない"
        );

        if let Focus::Route(route) = self {
            let mut route = route.clone();
            route.push(index);
            Focus::Route(route)
        } else {
            unreachable!()
        }
    }

    fn shift(&mut self) -> Option<usize> {
        assert_ne!(
            self,
            &Focus::Done,
            "すでに簡約が完了しているため index を取り出せない"
        );

        let Focus::Route(ref mut route) = self else {
            unreachable!()
        };
        route.pop()
    }
}

// ========================================================================== //

struct Args<'a>(Vec<Option<&'a mut Expr>>);

impl<'a> Args<'a> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, expr: &'a mut Expr) {
        self.0.push(Some(expr));
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn pop(&mut self, arity: usize) -> Option<Vec<&'a mut Expr>> {
        if self.len() >= arity {
            let args = self.0.split_off(self.len() - arity);
            Some(
                args.into_iter()
                    .map(|maybe_expr| maybe_expr.unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    fn into_iter(self) -> impl Iterator<Item = &'a mut Expr> {
        self.0.into_iter().map(|maybe_expr| maybe_expr.unwrap())
    }

    /// n 番目の引数を取得する、ただし末尾から数える
    fn nth(&mut self, index: usize) -> Option<&'a mut Expr> {
        let last_index = self.len() - 1;
        let index = last_index - index;
        self.0.get_mut(index).map(|expr| expr.take().unwrap())
    }
}

// impl<'a> From<Args<'a>> for Vec<&'a mut Expr> {
//     fn from(args: Args<'b>) -> Vec<&'b mut Expr> {
//         args.into_iter().collect()
//     }
// }

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

        Context::from(vec![i, k, s])
    }

    #[test]
    fn test_walk_to() {
        let context = setup();

        let focus = Focus::Route(vec![]);
        let mut expr = expr::a("i", "x");
        let mut expected = expr::v("i");
        assert_eq!(walk_to(focus, &mut expr), &mut expected);

        let focus = Focus::Route(vec![0]);
        let mut expr = expr::a("f", expr::a("i", "x"));
        let mut expected = expr::v("i");
        assert_eq!(walk_to(focus, &mut expr), &mut expected);
    }
}
