use crate::expr::Expr;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct FreeVars<'a>(pub HashSet<&'a str>);

impl FreeVars<'_> {
    pub fn contains<Id: AsRef<str>>(&self, id: &Id) -> bool {
        self.0.contains(id.as_ref())
    }
}

impl<'a> From<&'a Expr> for FreeVars<'a> {
    fn from(expr: &'a Expr) -> Self {
        let mut vars = HashSet::new();
        free_vars(expr, &mut vars);
        FreeVars(vars)
    }
}

fn free_vars<'a>(expr: &'a Expr, vars: &mut HashSet<&'a str>) {
    match expr {
        Expr::Variable(id) => {
            vars.insert(id.as_str());
        }

        Expr::Symbol(_) => {}

        Expr::Apply { lhs, rhs } => {
            free_vars(lhs.as_ref(), vars);
            free_vars(rhs.as_ref(), vars);
        }

        Expr::Lambda { param, body } => {
            let mut body_vars: HashSet<&'a str> = HashSet::new();
            free_vars(body.as_ref(), &mut body_vars);

            for var in body_vars {
                if var != param.as_str() {
                    vars.insert(var);
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
    fn test_free_vars() {
        let e = expr::v("x");
        let expected = ["x"];
        assert_eq!(FreeVars::from(&e), FreeVars::from(expected));

        let e = expr::s("x");
        let expected = [];
        assert_eq!(FreeVars::from(&e), FreeVars::from(expected));

        let e = expr::a("x", "y");
        let expected = ["x", "y"];
        assert_eq!(FreeVars::from(&e), FreeVars::from(expected));

        let e = expr::l("x", "x");
        let expected = [];
        assert_eq!(FreeVars::from(&e), FreeVars::from(expected));

        let e = expr::l("x", "y");
        let expected = ["y"];
        assert_eq!(FreeVars::from(&e), FreeVars::from(expected));

        let e = expr::l("x", expr::l("y", expr::a("x", "y")));
        let expected = [];
        assert_eq!(FreeVars::from(&e), FreeVars::from(expected));
    }

    impl<'a, const N: usize> From<[&'a str; N]> for FreeVars<'a> {
        fn from(ids: [&'a str; N]) -> Self {
            let vars = HashSet::from(ids);
            FreeVars(vars)
        }
    }
}
