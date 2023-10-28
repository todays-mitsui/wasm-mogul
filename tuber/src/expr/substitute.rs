use super::bound_vars::BoundVars;
use super::free_vars::FreeVars;
use crate::expr::{Expr, Identifier};

impl Expr {
    /// 指定した識別子を別の式で置き換えた新しい式を得る
    ///
    /// ラムダ抽象の中で束縛されている束縛変数と自由変数の衝突を避けるため
    /// 束縛変数のリネームを行うことがある (α変換)
    ///
    /// ```
    /// # use crate::expr::{self, Expr, Identifier};
    ///
    /// // ^y.`xy [x := y]
    /// let expr = expr::l("y", expr::a("x", "y"));
    /// let param = Identifier::from("x");
    /// let arg = expr::v("y");
    ///
    /// // 単純に x を y に置換した結果にはならない
    /// // そのようにしてしまうと自由変数としての y と束縛変数としての y の区別がつかなくなってしまう
    /// assert_ne!(
    ///     expr.clone().substitute(&param, arg),
    ///     // ^y.`yy
    ///     expr::l("y", expr::a("y", "y"))
    /// );
    ///
    /// // ^y.`xy [x := y] を ^Y.`xY [x := y] に変換することで自由変数と束縛変数の衝突を避ける
    /// assert_eq!(
    ///     expr.clone().substitute(&param, arg),
    ///     // ^Y.`xY
    ///     expr::l("Y", expr::a("y", "Y"))
    /// );
    /// ```
    pub fn substitute(&mut self, param: &Identifier, arg: &Expr) {
        let bound_vars = BoundVars::new();
        let free_vars = FreeVars::from(arg);
        self.substitute_impl(param, arg, &free_vars, bound_vars);
    }

    fn substitute_impl<'a>(
        &'a mut self,
        param: &Identifier,
        arg: &Expr,
        free_vars: &FreeVars,
        mut bound_vars: BoundVars,
    ) {
        match self {
            // param と同名の変数は arg に置き換える
            Expr::Variable(ref id) if id == param => {
                *self = arg.clone();
            }

            // さもなくば、そのまま返す
            Expr::Variable(_) => {}

            // シンボルは置換の対象にならない
            Expr::Symbol(_) => {}

            // 再帰的に置換を行う
            Expr::Apply { lhs, rhs } => {
                lhs.substitute_impl(param, arg, free_vars, bound_vars.clone());
                rhs.substitute_impl(param, arg, free_vars, bound_vars.clone());
            }

            // param と同名の引数を持つラムダ抽象は内部に自由変数としての param を持たない
            // そのため即座に検索を打ち切って良い
            Expr::Lambda {
                param: ref p,
                body: _,
            } if p == param => {}

            // arg の中の自由変数とラムダ抽象の引数 p が衝突する場合
            // ラムダ抽象の引数 p を適切にリネームする必要がある (α変換)
            // リネームしなければ引数としての p と自由変数としての p が区別できなくなってしまう
            Expr::Lambda {
                param: ref mut p,
                body,
            } => {
                if free_vars.contains(p) {
                    // p を適切にリネームする
                    // リネーム後の名前はどの束縛変数とも被ってはいけない
                    let new_p = p.rename(&bound_vars);

                    // body の中の全ての p をリネームした new_param に置き換える
                    replace(body, p, &new_p);

                    *p = new_p;
                }

                bound_vars.insert(p.clone());

                // 再帰的に置換を行う
                body.substitute_impl(param, arg, free_vars, bound_vars);
            }
        }
    }
}

/// 式の中の自由変数を別の識別子に置き換える
fn replace(expr: &mut Expr, old: &Identifier, new: &Identifier) {
    match expr {
        Expr::Variable(id) => {
            if id == old {
                *id = new.clone();
            }
        }

        Expr::Symbol(_) => {}

        Expr::Apply { lhs, rhs } => {
            replace(lhs.as_mut(), old, new);
            replace(rhs.as_mut(), old, new);
        }

        Expr::Lambda { param, body } => {
            if param != old {
                replace(body.as_mut(), old, new);
            } else {
                // 何もしない
                //
                // 自由変数としての old のみ new に置き換えたい
                // old が束縛変数の識別子と一致する場合、そのラムダ抽象の中に自由変数としての old は
                // 存在しないことが確定するので、その時点で再起を打ち切っていい
            }
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    #[test]
    fn test_expr_substitute() {
        // ^z.x [x := y] => ^z.y
        let mut e = expr::l("z", "x");
        e.substitute(&"x".into(), &"y".into());
        assert_eq!(e, expr::l("z", "y"));

        // ^Y.^y.`xY [x := y] => ^Y.^Y0.`yY
        let mut e = expr::l("Y", expr::l("y", expr::a("x", "Y")));
        e.substitute(&"x".into(), &"y".into());
        assert_eq!(e, expr::l("Y", expr::l("Y0", expr::a("y", "Y"))));
    }

    #[test]
    /// ラムダ抽象の中で束縛されている変数は置換されない
    fn test_replace_1() {
        let mut expr = expr::l("x", expr::a("x", "y"));
        let expected = expr::l("x", expr::a("x", "y"));

        replace(&mut expr, &"x".into(), &"a".into());

        assert_eq!(expr, expected);
    }

    #[test]
    /// 置換はラムダ抽象の中にまで渡って再起的に行われる
    fn test_replace_2() {
        let mut expr = expr::l("x", expr::a("x", "y"));
        let expected = expr::l("x", expr::a("x", "a"));

        replace(&mut expr, &"y".into(), &"a".into());

        assert_eq!(expr, expected);
    }

    #[test]
    /// 置換は左右の枝に渡って再起的に行われる
    fn test_rename_var_3() {
        let mut expr = expr::a(expr::a("x", "y"), expr::a("y", "x"));
        let expected = expr::a(expr::a("a", "y"), expr::a("y", "a"));

        replace(&mut expr, &"x".into(), &"a".into());

        assert_eq!(expr, expected);
    }

    #[test]
    /// 変数とシンボルは区別される
    /// :x が x によって置換されることはない
    fn test_rename_var_4() {
        let mut expr = expr::a(expr::a(":x", "y"), expr::a("y", ":x"));
        let expected = expr::a(expr::a(":x", "y"), expr::a("y", ":x"));

        replace(&mut expr, &"x".into(), &"a".into());

        assert_eq!(expr, expected);
    }
}
