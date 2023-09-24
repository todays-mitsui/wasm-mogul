use super::super::style::ECMAScriptStyle;
use crate::func::Func;
use std::fmt::Display;

impl Display for ECMAScriptStyle<'_, Func> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func: &Func = self.0;
        if func.arity() == 0 {
            write!(f, "{} = {}", func.name(), ECMAScriptStyle(&func.body()))
        } else {
            write!(
                f,
                "{}({}) = {}",
                func.name(),
                func.params()
                    .iter()
                    .map(|i| i.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                ECMAScriptStyle(&func.body())
            )
        }
    }
}

impl Display for ECMAScriptStyle<'_, &Func> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func: &Func = self.0;
        if func.arity() == 0 {
            write!(f, "{} = {}", func.name(), ECMAScriptStyle(func.body()))
        } else {
            write!(
                f,
                "{}({}) = {}",
                func.name(),
                func.params()
                    .iter()
                    .map(|i| i.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                ECMAScriptStyle(func.body())
            )
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::func;

    #[test]
    fn test_to_string() {
        let f = func::new("TRUE", Vec::<&str>::new(), "k");
        assert_eq!(ECMAScriptStyle(&f).to_string(), "TRUE = k");

        let f = func::new("f", vec!["x", "y"], expr::a("x", "y"));
        assert_eq!(ECMAScriptStyle(&f).to_string(), "f(x, y) = x(y)");

        let f = func::new("F", vec!["X", "Y"], expr::a("X", "Y"));
        assert_eq!(ECMAScriptStyle(&f).to_string(), "F(X, Y) = X(Y)");

        let f = func::new("F", vec!["x", "Y"], expr::a("x", "Y"));
        assert_eq!(ECMAScriptStyle(&f).to_string(), "F(x, Y) = x(Y)");
    }
}
