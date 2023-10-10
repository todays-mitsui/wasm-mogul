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
            Command::Global => write!(f, "?"),
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
            Command::Global => write!(f, "?"),
            Command::Unlambda(level, e) => {
                write!(f, "{} {}", "~".repeat((*level).into()), LazyKStyle(e))
            }
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use crate::engine::command;
    use crate::expr;
    use crate::func;

    #[test]
    fn test_del() {
        assert_eq!(command::del("i").to_string(), "i = i");
    }

    #[test]
    fn test_update() {
        assert_eq!(
            command::update(func::new("i", vec!["x"], "x")).to_string(),
            "`ix = x"
        );

        assert_eq!(
            command::update(func::new("k", vec!["x", "y"], "x")).to_string(),
            "``kxy = x"
        );

        assert_eq!(
            command::update(func::new(
                "s",
                vec!["x", "y", "z"],
                expr::a(expr::a("x", "z"), expr::a("y", "z"))
            ))
            .to_string(),
            "```sxyz = ``xz`yz"
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(command::eval(expr::v("a")).to_string(), "a");
        assert_eq!(command::eval(expr::s("a")).to_string(), ":a");
        assert_eq!(command::eval(expr::a("a", "b")).to_string(), "`ab");
        assert_eq!(command::eval(expr::l("x", "y")).to_string(), "λx.y");
    }

    #[test]
    fn test_eval_last() {
        assert_eq!(command::eval_last(expr::v("a")).to_string(), "! a");
        assert_eq!(command::eval_last(expr::s("a")).to_string(), "! :a");
        assert_eq!(command::eval_last(expr::a("a", "b")).to_string(), "! `ab");
        assert_eq!(command::eval_last(expr::l("x", "y")).to_string(), "! λx.y");
    }

    #[test]
    fn test_eval_head() {
        assert_eq!(command::eval_head(42, expr::v("a")).to_string(), "!42 a");
        assert_eq!(command::eval_head(42, expr::s("a")).to_string(), "!42 :a");
        assert_eq!(
            command::eval_head(42, expr::a("a", "b")).to_string(),
            "!42 `ab"
        );
        assert_eq!(
            command::eval_head(42, expr::l("x", "y")).to_string(),
            "!42 λx.y"
        );
    }

    #[test]
    fn test_eval_tail() {
        assert_eq!(command::eval_tail(42, expr::v("a")).to_string(), "!-42 a");
        assert_eq!(command::eval_tail(42, expr::s("a")).to_string(), "!-42 :a");
        assert_eq!(
            command::eval_tail(42, expr::a("a", "b")).to_string(),
            "!-42 `ab"
        );
        assert_eq!(
            command::eval_tail(42, expr::l("x", "y")).to_string(),
            "!-42 λx.y"
        );
    }

    #[test]
    fn test_search() {
        assert_eq!(command::search("i").to_string(), "? i");
    }

    #[test]
    fn test_global() {
        assert_eq!(command::global().to_string(), "?");
    }
}
