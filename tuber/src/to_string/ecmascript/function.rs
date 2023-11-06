use super::expression;
use crate::func::Func;

pub fn to_string(func: &Func) -> String {
    if func.arity() == 0 {
        format!("{} = {}", func.name(), expression::to_string(func.body()))
    } else {
        format!(
            "{}({}) = {}",
            func.name(),
            func.params()
                .iter()
                .map(|i| i.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            expression::to_string(func.body())
        )
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
        assert_eq!(to_string(&f), "TRUE = k");

        let f = func::new("f", vec!["x", "y"], expr::a("x", "y"));
        assert_eq!(to_string(&f), "f(x, y) = x(y)");

        let f = func::new("F", vec!["X", "Y"], expr::a("X", "Y"));
        assert_eq!(to_string(&f), "F(X, Y) = X(Y)");

        let f = func::new("F", vec!["x", "Y"], expr::a("x", "Y"));
        assert_eq!(to_string(&f), "F(x, Y) = x(Y)");
    }
}
