use super::apply::apply;
use crate::calc;
use crate::context::Context;
use crate::expr::{self, Path, PathBuilder};

pub struct Reducer {
    context: Context,
    step: usize,
    expr: Expr,
}

pub struct ReduceResult {
    pub step: usize,
    pub expr: expr::Expr,
    pub reduced_path: Path,
}

impl Reducer {
    pub fn new(context: Context, expr: expr::Expr) -> Self {
        let expr = Expr::new(&context, expr);
        Self {
            context,
            step: 0,
            expr,
        }
    }

    pub fn reducible_path(&self) -> Option<Path> {
        self.expr.reducible_path(&self.context)
    }
}

impl Iterator for Reducer {
    type Item = ReduceResult;

    fn next(&mut self) -> Option<Self::Item> {
        let reducible_path = self.expr.reducible_path(&self.context)?;

        let reduced_path = self.expr.reduce(&self.context, &reducible_path);
        self.step += 1;

        Some(ReduceResult {
            step: self.step,
            expr: expr::Expr::from(self.expr.to_owned()),
            reduced_path,
        })
    }
}

#[derive(Clone)]
struct Expr {
    callee: expr::Expr,
    args: Vec<Expr>,
}

impl Expr {
    fn new(context: &Context, expr: expr::Expr) -> Self {
        let mut callee: expr::Expr = expr;
        let mut args: Vec<Expr> = Vec::new();

        while let expr::Expr::Apply { lhs, rhs } = callee {
            callee = *lhs;
            args.push(Expr::new(context, *rhs));
        }

        Self { callee, args }
    }

    fn arity(&self, context: &Context) -> Option<usize> {
        calc::arity(context, &self.callee)
    }

    // self.callee に self.args のうちいくつかの項を与えて簡約可能かどうかを判定する
    fn callable(&self, context: &Context) -> bool {
        // self.callee が arity = 0 の関数であっても、引数を伴わない単項の式なら簡約したくない
        // そのため簡約可能であるためには少なくとも1つ以上の引数を持つべきだ
        if self.args.len() == 0 {
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
    fn reducible(&self, context: &Context) -> bool {
        // TODO: reducible() が呼び出されるたびに args を再帰的に辿っていくのが非効率かもしれない、計算結果をキャッシュする機構を考えたい
        self.callable(context) || self.args.iter().any(|arg| arg.reducible(context))
    }

    // 簡約基に至る経路を返す
    fn reducible_path(&self, context: &Context) -> Option<Path> {
        let mut path = PathBuilder::new();
        let mut expr = self;

        loop {
            if expr.callable(context) {
                path.set_arity(expr.arity(context).unwrap());
                return Some(path.build());
            } else {
                match expr
                    .args
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, arg)| arg.reducible(context))
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
            expr.args.push(Expr::new(context, *rhs.to_owned()));
        }
        expr.callee = callee.to_owned();

        let mut reduced_path = reducible_path.clone();
        reduced_path.set_arity(num_args);
        return reduced_path;
    }
}

impl From<Expr> for expr::Expr {
    fn from(expr: Expr) -> expr::Expr {
        let mut e = expr.callee;
        for arg in expr.args {
            e = expr::a(e, expr::Expr::from(arg));
        }
        e
    }
}
