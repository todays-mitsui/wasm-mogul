use crate::engine::Command;
use crate::style::LazyKStyle;
use std::fmt::Display;

impl Display for LazyKStyle<'_, Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &Command = self.0;
        match command {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", LazyKStyle(func)),
            Command::Eval(e) => write!(f, "{}", LazyKStyle(e)),
            Command::EvalLast(e) => write!(f, "! {}", e),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, LazyKStyle(e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, LazyKStyle(e)),
            Command::Search(i) => write!(f, "? {}", i),
            Command::Context => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), LazyKStyle(e))
            }
        }
    }
}

impl Display for LazyKStyle<'_, &Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command: &Command = self.0;
        match command {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", LazyKStyle(func)),
            Command::Eval(e) => write!(f, "{}", LazyKStyle(e)),
            Command::EvalLast(e) => write!(f, "! {}", e),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, LazyKStyle(e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, LazyKStyle(e)),
            Command::Search(i) => write!(f, "? {}", i),
            Command::Context => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), LazyKStyle(e))
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
        assert_eq!(LazyKStyle(&command).to_string(), "i = i");
    }

    #[test]
    fn test_update() {
        let command = command::update(func::new("i", vec!["x"], "x"));
        assert_eq!(LazyKStyle(&command).to_string(), "`ix = x");

        let command = command::update(func::new("k", vec!["x", "y"], "x"));
        assert_eq!(LazyKStyle(&command).to_string(), "``kxy = x");

        let command = command::update(func::new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        ));
        assert_eq!(LazyKStyle(&command).to_string(), "```sxyz = ``xz`yz");
    }

    #[test]
    fn test_eval() {
        let command = command::eval(expr::v("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "a");

        let command = command::eval(expr::s("a"));
        assert_eq!(LazyKStyle(&command).to_string(), ":a");

        let command = command::eval(expr::a("a", "b"));
        assert_eq!(LazyKStyle(&command).to_string(), "`ab");

        let command = command::eval(expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "λx.y");
    }

    #[test]
    fn test_eval_last() {
        let command = command::eval_last(expr::v("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "! a");

        let command = command::eval_last(expr::s("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "! :a");

        let command = command::eval_last(expr::a("a", "b"));
        assert_eq!(LazyKStyle(&command).to_string(), "! `ab");

        let command = command::eval_last(expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "! λx.y");
    }

    #[test]
    fn test_eval_head() {
        let command = command::eval_head(42, expr::v("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "!42 a");

        let command = command::eval_head(42, expr::s("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "!42 :a");

        let command = command::eval_head(42, expr::a("a", "b"));
        assert_eq!(LazyKStyle(&command).to_string(), "!42 `ab");

        let command = command::eval_head(42, expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "!42 λx.y");
    }

    #[test]
    fn test_eval_tail() {
        let command = command::eval_tail(42, expr::v("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "!-42 a");

        let command = command::eval_tail(42, expr::s("a"));
        assert_eq!(LazyKStyle(&command).to_string(), "!-42 :a");

        let command = command::eval_tail(42, expr::a("a", "b"));
        assert_eq!(LazyKStyle(&command).to_string(), "!-42 `ab");

        let command = command::eval_tail(42, expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "!-42 λx.y");
    }

    #[test]
    fn test_search() {
        let command = command::search("i");
        assert_eq!(LazyKStyle(&command).to_string(), "? i");
    }

    #[test]
    fn test_global() {
        let command = command::context();
        assert_eq!(LazyKStyle(&command).to_string(), "?");
    }

    #[test]
    fn test_unlambda() {
        let command = command::unlambda(1, expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "~ λx.y");

        let command = command::unlambda(2, expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "~~ λx.y");

        let command = command::unlambda(3, expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "~~~ λx.y");

        let command = command::unlambda(4, expr::l("x", "y"));
        assert_eq!(LazyKStyle(&command).to_string(), "~~~~ λx.y");
    }
}
