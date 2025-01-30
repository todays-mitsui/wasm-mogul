use crate::calc::aliases::Aliases;
use crate::context::Context;
use crate::expr::Expr;
use anyhow::{anyhow, Result};

pub fn apply(context: &Context, aliases: &Aliases, expr: &mut Expr, args: Vec<Expr>) -> Result<()> {
    if let Some(arity) = arity(context, aliases, expr) {
        if arity != args.len() {
            return Err(anyhow!(
                "Number of arguments is mismatch: {} arg(s) expected",
                arity
            ));
        }
    }

    match expr {
        Expr::Lambda { param, body } => {
            body.substitute(param, &args[0]);
            *expr = *body.clone();
            Ok(())
        }

        Expr::Variable(id) => {
            if let Some(alias) = aliases.get(id) {
                *expr = alias.to_owned();
                return Ok(());
            }

            return match context.get(id) {
                Some(func) => {
                    *expr = func.apply(args);
                    Ok(())
                }
                None => Err(anyhow!("Undefined function: {}", id)),
            };
        }

        _ => Err(anyhow!("Not a function: {}", expr)),
    }
}

fn arity(context: &Context, aliases: &Aliases, expr: &Expr) -> Option<usize> {
    match expr {
        Expr::Lambda { .. } => Some(1),
        Expr::Variable(id) => {
            if aliases.has(id) {
                return Some(0);
            }
            context.get(id).map(|f| f.arity())
        }
        _ => None,
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::func;
    use crate::Identifier;
    use std::collections::HashMap;

    fn setup() -> (Context, Aliases) {
        let i = func::new("i", vec!["x"], "x");
        let k = func::new("k", vec!["x", "y"], "x");
        let x = func::new("x", vec!["x"], expr::a("x", "x"));

        let mut aliases: HashMap<Identifier, Expr> = HashMap::new();
        aliases.insert("_".into(), expr::l("x", expr::a("x", "x")));
        aliases.insert("_1".into(), "TRUE".into());
        aliases.insert("_2".into(), "FALSE".into());

        (Context::from(vec![i, k, x]), aliases.into())
    }

    #[test]
    fn test_aliases() {
        let (context, aliases) = setup();

        // _ => ^x.`xx
        let mut expr = expr::v("_");
        let args = vec![];
        let result = apply(&context, &aliases, &mut expr, args);

        assert!(result.is_ok());
        assert_eq!(expr, expr::l("x", expr::a("x", "x")));

        // _2 => FALSE
        let mut expr = expr::v("_2");
        let args = vec![];
        let result = apply(&context, &aliases, &mut expr, args);

        assert!(result.is_ok());
        assert_eq!(expr, "FALSE".into());
    }

    #[test]
    fn test_apply_ok() {
        let (context, aliases) = setup();

        // `i:a => :a
        let mut expr = expr::v("i");
        let args = vec![expr::s("a")];
        let result = apply(&context, &aliases, &mut expr, args);

        assert!(result.is_ok());
        assert_eq!(expr, expr::s("a"));

        // ``k:a:b => :a
        let mut expr = expr::v("k");
        let args = vec![expr::s("a"), expr::s("b")];
        let result = apply(&context, &aliases, &mut expr, args);

        assert!(result.is_ok());
        assert_eq!(expr, expr::s("a"));

        // `x:a => `:a:a
        let mut expr = expr::v("x");
        let args = vec![expr::s("a")];
        let result = apply(&context, &aliases, &mut expr, args);

        assert!(result.is_ok());
        assert_eq!(expr, expr::a(":a", ":a"));

        // `^x.:a:b => :a
        let mut expr = expr::l("x", ":a");
        let args = vec![expr::s("b")];
        let result = apply(&context, &aliases, &mut expr, args);

        assert!(result.is_ok());
        assert_eq!(expr, expr::s("a"));
    }

    #[test]
    fn test_apply_err() {
        let (context, aliases) = setup();

        // `:i:a
        let mut expr = expr::s("i");
        let args = vec![expr::s("a")];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // シンボルは関数に紐づくことがない

        // `y:a
        let mut expr = expr::v("y");
        let args = vec![expr::s("a")];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // context に存在しない不明な識別子に対して適用を試みた

        // `(`i:a):b
        let mut expr = expr::a("i", ":a");
        let args = vec![expr::s("b")];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // apply() は ``(i):a:b 形式しか受け入れない

        // `k:a
        let mut expr = expr::v("k");
        let args = vec![expr::s("a")];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // k の arity が 2 なのに対して引数の個数が少ない

        // ```k:a:b:c
        let mut expr = expr::v("k");
        let args = vec![expr::s("a"), expr::s("b"), expr::s("c")];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // k の arity が 2 なのに対して引数の個数が多すぎる

        // ^x.:a
        let mut expr = expr::l("x", ":a");
        let args = vec![];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // ラムダ抽象の arity が 1 なのに対して引数の個数が少ない

        // ``^x.:a:b:c
        let mut expr = expr::l("x", ":a");
        let args = vec![expr::s("b"), expr::s("c")];
        let result = apply(&context, &aliases, &mut expr, args);
        assert!(result.is_err()); // ラムダ抽象の arity が 1 なのに対して引数の個数が多すぎる
    }

    #[test]
    fn test_arity() {
        let f0 = func::new("F0", Vec::<&str>::new(), ":a");
        let f1 = func::new("F1", vec!["x"], ":a");
        let f2 = func::new("F2", vec!["x", "y"], ":a");
        let f3 = func::new("F3", vec!["x", "y", "z"], ":a");

        let context = Context::from(vec![f0, f1, f2, f3]);
        let aliases = Aliases::new();

        // シンボルは関数が紐づくことがない、arity は定義されない
        let e = expr::s("a");
        assert_eq!(arity(&context, &aliases, &e), None);

        // 関数適用の arity は定義されない
        let e = expr::a("x", "y");
        assert_eq!(arity(&context, &aliases, &e), None);

        // ラムダ抽象の arity は常に 1
        let e = expr::l("x", "x");
        assert_eq!(arity(&context, &aliases, &e), Some(1));

        // 関数として定義されていない自由変数の arity は定義されない (0ですらない)
        let e = expr::v("x");
        assert_eq!(arity(&context, &aliases, &e), None);

        // 定義済み関数と紐づく自由変数はその関数の arity を返す
        let e = expr::v("F0");
        assert_eq!(arity(&context, &aliases, &e), Some(0));
        let e = expr::v("F1");
        assert_eq!(arity(&context, &aliases, &e), Some(1));
        let e = expr::v("F2");
        assert_eq!(arity(&context, &aliases, &e), Some(2));
        let e = expr::v("F3");
        assert_eq!(arity(&context, &aliases, &e), Some(3));
    }

    #[test]
    fn test_apply() {
        // TODO: テスト書く
    }
}
