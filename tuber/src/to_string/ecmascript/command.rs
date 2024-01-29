use super::expression;
use super::function;
use crate::engine::Command;

pub fn to_string(command: &Command) -> String {
    match command {
        Command::Del(i) => format!("{} = {}", i, i),
        Command::Update(func) => function::to_string(func),
        Command::Eval(e) => expression::to_string(e),
        Command::EvalLast(e) => format!("! {}", expression::to_string(e)),
        Command::EvalHead(len, e) => format!("!{} {}", len, expression::to_string(e)),
        Command::EvalTail(len, e) => format!("!-{} {}", len, expression::to_string(e)),
        Command::Query(i) => format!("? {}", i),
        Command::Context => "?".to_string(),
        Command::Unlambda(level, e) => {
            format!(
                "{} {}",
                "~".repeat((*level).into()),
                expression::to_string(e)
            )
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::command;
    use crate::expr;
    use crate::func;

    #[test]
    fn test_del() {
        let command = command::del("i");
        assert_eq!(to_string(&command), "i = i");
    }

    #[test]
    fn test_update() {
        let command = command::update(func::new("i", vec!["x"], "x"));
        assert_eq!(to_string(&command), "i(x) = x");

        let command = command::update(func::new("k", vec!["x", "y"], "x"));
        assert_eq!(to_string(&command), "k(x, y) = x");

        let command = command::update(func::new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        ));
        assert_eq!(to_string(&command), "s(x, y, z) = x(z, y(z))");
    }

    #[test]
    fn test_eval() {
        let command = command::eval(expr::v("a"));
        assert_eq!(to_string(&command), "a");

        let command = command::eval(expr::s("a"));
        assert_eq!(to_string(&command), ":a");

        let command = command::eval(expr::a("a", "b"));
        assert_eq!(to_string(&command), "a(b)");

        let command = command::eval(expr::l("x", "y"));
        assert_eq!(to_string(&command), "x => y");
    }

    #[test]
    fn test_eval_last() {
        let command = command::eval_last(expr::v("a"));
        assert_eq!(to_string(&command), "! a");

        let command = command::eval_last(expr::s("a"));
        assert_eq!(to_string(&command), "! :a");

        let command = command::eval_last(expr::a("a", "b"));
        assert_eq!(to_string(&command), "! a(b)");

        let command = command::eval_last(expr::l("x", "y"));
        assert_eq!(to_string(&command), "! x => y");
    }

    #[test]
    fn test_eval_head() {
        let command = command::eval_head(42, expr::v("a"));
        assert_eq!(to_string(&command), "!42 a");

        let command = command::eval_head(42, expr::s("a"));
        assert_eq!(to_string(&command), "!42 :a");

        let command = command::eval_head(42, expr::a("a", "b"));
        assert_eq!(to_string(&command), "!42 a(b)");

        let command = command::eval_head(42, expr::l("x", "y"));
        assert_eq!(to_string(&command), "!42 x => y");
    }

    #[test]
    fn test_eval_tail() {
        let command = command::eval_tail(42, expr::v("a"));
        assert_eq!(to_string(&command), "!-42 a");

        let command = command::eval_tail(42, expr::s("a"));
        assert_eq!(to_string(&command), "!-42 :a");

        let command = command::eval_tail(42, expr::a("a", "b"));
        assert_eq!(to_string(&command), "!-42 a(b)");

        let command = command::eval_tail(42, expr::l("x", "y"));
        assert_eq!(to_string(&command), "!-42 x => y");
    }

    #[test]
    fn test_query() {
        let command = command::query("i");
        assert_eq!(to_string(&command), "? i");
    }

    #[test]
    fn test_global() {
        let command = command::context();
        assert_eq!(to_string(&command), "?");
    }

    #[test]
    fn test_unlambda() {
        let command = command::unlambda(1, expr::l("x", "y"));
        assert_eq!(to_string(&command), "~ x => y");

        let command = command::unlambda(2, expr::l("x", "y"));
        assert_eq!(to_string(&command), "~~ x => y");

        let command = command::unlambda(3, expr::l("x", "y"));
        assert_eq!(to_string(&command), "~~~ x => y");

        let command = command::unlambda(4, expr::l("x", "y"));
        assert_eq!(to_string(&command), "~~~~ x => y");
    }
}
