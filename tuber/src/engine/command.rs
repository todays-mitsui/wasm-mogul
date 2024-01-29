use crate::expr::{Expr, Identifier};
use crate::func::Func;

// TODO: Func や Expr を Box に入れたほうがいいかも
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Del(Identifier), // 関数を削除
    // Add(Ident, Func),      // 関数定義 (定義済み関数の上書きを許さない)
    Update(Func),          // 関数定義 (定義済み関数の上書きを許す)
    Eval(Expr),            // β変換列を表示
    EvalLast(Expr),        // β変結果のみ表示
    EvalHead(usize, Expr), // β変換列の先頭のみ表示
    EvalTail(usize, Expr), // β変換列の末尾のみ表示
    Query(Identifier),     // Context から定義済み関数を検索
    Context,               // Context 全体を表示
    Unlambda(u8, Expr),    // Expr からラムダ抽象を除去する
}

#[cfg(test)]
pub fn del<Id: Into<Identifier>>(id: Id) -> Command {
    Command::Del(id.into())
}

#[cfg(test)]
pub fn update(func: Func) -> Command {
    Command::Update(func)
}

#[cfg(test)]
pub fn eval<E: Into<Expr>>(expr: E) -> Command {
    Command::Eval(expr.into())
}

#[cfg(test)]
pub fn eval_last<E: Into<Expr>>(expr: E) -> Command {
    Command::EvalLast(expr.into())
}

#[cfg(test)]
pub fn eval_head<E: Into<Expr>>(n: usize, expr: E) -> Command {
    Command::EvalHead(n, expr.into())
}

#[cfg(test)]
pub fn eval_tail<E: Into<Expr>>(n: usize, expr: E) -> Command {
    Command::EvalTail(n, expr.into())
}

#[cfg(test)]
pub fn query<Id: Into<Identifier>>(id: Id) -> Command {
    Command::Query(id.into())
}

#[cfg(test)]
pub fn context() -> Command {
    Command::Context
}

#[cfg(test)]
pub fn unlambda<E: Into<Expr>>(level: u8, expr: E) -> Command {
    Command::Unlambda(level, expr.into())
}
