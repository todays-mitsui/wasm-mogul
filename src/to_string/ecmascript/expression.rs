use crate::expr::Expr;
use crate::style::ECMAScriptStyle;
use std::fmt::Display;

impl Display for ECMAScriptStyle<'_, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr: &Expr = self.0;
        write!(f, "{}", Compact::new(expr).to_string())
    }
}

impl Display for ECMAScriptStyle<'_, &Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr: &Expr = self.0;
        write!(f, "{}", Compact::new(expr).to_string())
    }
}

// ========================================================================== //

enum Compact<'a> {
    Variable(&'a str),
    Symbol(&'a str),
    Apply(Box<Compact<'a>>, Vec<Compact<'a>>),
    Lambda(Vec<&'a str>, Box<Compact<'a>>),
}

impl<'a> Compact<'a> {
    fn new(e: &'a Expr) -> Compact<'a> {
        match e {
            Expr::Variable(i) => {
                let label = i.as_str();
                Compact::Variable(label)
            }

            Expr::Symbol(i) => {
                let label = i.as_str();
                Compact::Symbol(label)
            }

            Expr::Apply { lhs, rhs } => {
                let e1 = Compact::new(lhs);
                let e2 = Compact::new(rhs);
                match e1 {
                    Compact::Apply(e1, mut es) => {
                        es.push(e2);
                        Compact::Apply(e1, es)
                    }
                    _ => Compact::Apply(Box::new(e1), vec![e2]),
                }
            }

            Expr::Lambda { param, body } => {
                let param = param.as_str();
                let body = Compact::new(body);
                match body {
                    Compact::Lambda(mut params, body) => {
                        params.push(param);
                        Compact::Lambda(params, body)
                    }
                    _ => Compact::Lambda(vec![&param], Box::new(body)),
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Compact::Variable(label) => format!("{}", label),

            Compact::Symbol(label) => format!(":{}", label),

            Compact::Apply(e, args) => {
                let args = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                match **e {
                    Compact::Variable(label) | Compact::Symbol(label) => {
                        format!("{}({})", label, args)
                    }

                    _ => {
                        format!("({})({})", e.to_string(), args)
                    }
                }
            }

            Compact::Lambda(params, body) => {
                if params.len() == 1 {
                    return format!("{} => {}", params[0], body.to_string());
                } else {
                    format!(
                        "({}) => {}",
                        params
                            .iter()
                            .map(|arg| arg.to_string())
                            .rev()
                            .collect::<Vec<_>>()
                            .join(", "),
                        body.to_string()
                    )
                }
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
    fn test_to_string() {
        let e = expr::v("x");
        assert_eq!(ECMAScriptStyle(&e).to_string(), "x");

        let e = expr::s("a");
        assert_eq!(ECMAScriptStyle(&e).to_string(), ":a");

        let e = expr::a("x", "y");
        assert_eq!(ECMAScriptStyle(&e).to_string(), "x(y)");

        let e = expr::a(expr::a("x", "y"), "z");
        assert_eq!(ECMAScriptStyle(&e).to_string(), "x(y, z)");

        let e = expr::a("x", expr::a("y", "z"));
        assert_eq!(ECMAScriptStyle(&e).to_string(), "x(y(z))");

        let e = expr::l("x", "a");
        assert_eq!(ECMAScriptStyle(&e).to_string(), "x => a");

        let e = expr::l("x", expr::l("y", "a"));
        assert_eq!(ECMAScriptStyle(&e).to_string(), "(x, y) => a");

        let e = expr::a(expr::l("x", "a"), "y");
        assert_eq!(ECMAScriptStyle(&e).to_string(), "(x => a)(y)");
    }
}
