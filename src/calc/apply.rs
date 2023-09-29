use crate::context::Context;
use crate::expr::Expr;
use anyhow::{anyhow, Result};

pub fn apply(context: &Context, expr: &mut Expr, args: Vec<Expr>) -> Result<()> {
    match expr {
        Expr::Lambda { param, body } => {
            body.substitute(param, &args[0]);
            Ok(())
        }

        Expr::Variable(id) => match context.get(id) {
            Some(func) => {
                *expr = func.apply(args);
                Ok(())
            }
            None => Err(anyhow!("Undefined function: {}", id)),
        },

        _ => Err(anyhow!("Not a function: {}", expr)),
    }
}

fn arity(context: &Context, expr: &Expr) -> Option<usize> {
    match expr {
        Expr::Lambda { .. } => Some(1),
        Expr::Variable(id) => context.get(id).map(|f| f.arity()),
        _ => None,
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::func;

    #[test]
    fn test_arity() {
        let f0 = func::new("F0", Vec::<&str>::new(), ":a");
        let f1 = func::new("F1", vec!["x"], ":a");
        let f2 = func::new("F2", vec!["x", "y"], ":a");
        let f3 = func::new("F3", vec!["x", "y", "z"], ":a");

        let context = Context::from(vec![f0, f1, f2, f3]);

        // シンボルは関数が紐づくことがない、arity は定義されない
        let e = expr::s("a");
        assert_eq!(arity(&context, &e), None);

        // 関数適用の arity は定義されない
        let e = expr::a("x", "y");
        assert_eq!(arity(&context, &e), None);

        // ラムダ抽象の arity は常に 1
        let e = expr::l("x", "x");
        assert_eq!(arity(&context, &e), Some(1));

        // 関数として定義されていない自由変数の arity は定義されない (0ですらない)
        let e = expr::v("x");
        assert_eq!(arity(&context, &e), None);

        // 定義済み関数と紐づく自由変数はその関数の arity を返す
        let e = expr::v("F0");
        assert_eq!(arity(&context, &e), Some(0));
        let e = expr::v("F1");
        assert_eq!(arity(&context, &e), Some(1));
        let e = expr::v("F2");
        assert_eq!(arity(&context, &e), Some(2));
        let e = expr::v("F3");
        assert_eq!(arity(&context, &e), Some(3));
    }

    #[test]
    fn test_apply() {
        // TODO: テスト書く
    }
}
