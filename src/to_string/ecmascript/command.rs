use crate::engine::Command;
use crate::style::ECMAScriptStyle;
use std::fmt::Display;

impl Display for ECMAScriptStyle<'_, Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &Command = self.0;
        match command {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", ECMAScriptStyle(func)),
            Command::Eval(e) => write!(f, "{}", ECMAScriptStyle(e)),
            Command::EvalLast(e) => write!(f, "! {}", ECMAScriptStyle(e)),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, ECMAScriptStyle(e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, ECMAScriptStyle(e)),
            Command::Search(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), ECMAScriptStyle(e))
            }
        }
    }
}

impl Display for ECMAScriptStyle<'_, &Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &Command = self.0;
        match command {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", ECMAScriptStyle(func)),
            Command::Eval(e) => write!(f, "{}", ECMAScriptStyle(e)),
            Command::EvalLast(e) => write!(f, "! {}", ECMAScriptStyle(e)),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, ECMAScriptStyle(e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, ECMAScriptStyle(e)),
            Command::Search(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), ECMAScriptStyle(e))
            }
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
        assert_eq!(ECMAScriptStyle(&command).to_string(), "i = i");
    }

    #[test]
    fn test_update() {
        let command = command::update(func::new("i", vec!["x"], "x"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "i(x) = x");

        let command = command::update(func::new("k", vec!["x", "y"], "x"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "k(x, y) = x");

        let command = command::update(func::new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        ));
        assert_eq!(
            ECMAScriptStyle(&command).to_string(),
            "s(x, y, z) = x(z, y(z))"
        );
    }

    #[test]
    fn test_eval() {
        let command = command::eval(expr::v("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "a");

        let command = command::eval(expr::s("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), ":a");

        let command = command::eval(expr::a("a", "b"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "a(b)");

        let command = command::eval(expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "x => y");
    }

    #[test]
    fn test_eval_last() {
        let command = command::eval_last(expr::v("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "! a");

        let command = command::eval_last(expr::s("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "! :a");

        let command = command::eval_last(expr::a("a", "b"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "! a(b)");

        let command = command::eval_last(expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "! x => y");
    }

    #[test]
    fn test_eval_head() {
        let command = command::eval_head(42, expr::v("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!42 a");

        let command = command::eval_head(42, expr::s("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!42 :a");

        let command = command::eval_head(42, expr::a("a", "b"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!42 a(b)");

        let command = command::eval_head(42, expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!42 x => y");
    }

    #[test]
    fn test_eval_tail() {
        let command = command::eval_tail(42, expr::v("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!-42 a");

        let command = command::eval_tail(42, expr::s("a"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!-42 :a");

        let command = command::eval_tail(42, expr::a("a", "b"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!-42 a(b)");

        let command = command::eval_tail(42, expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "!-42 x => y");
    }

    #[test]
    fn test_search() {
        let command = command::search("i");
        assert_eq!(ECMAScriptStyle(&command).to_string(), "? i");
    }

    #[test]
    fn test_global() {
        let command = command::global();
        assert_eq!(ECMAScriptStyle(&command).to_string(), "?");
    }

    #[test]
    fn test_unlambda() {
        let command = command::unlambda(1, expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "~ x => y");

        let command = command::unlambda(2, expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "~~ x => y");

        let command = command::unlambda(3, expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "~~~ x => y");

        let command = command::unlambda(4, expr::l("x", "y"));
        assert_eq!(ECMAScriptStyle(&command).to_string(), "~~~~ x => y");
    }
}
