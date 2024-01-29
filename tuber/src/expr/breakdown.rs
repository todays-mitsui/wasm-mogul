use super::{Expr, Identifier};

impl Expr {
    pub fn unapply(&self) -> (&Expr, Vec<&Expr>) {
        let mut callee: &Expr = self;
        let mut args: Vec<&Expr> = Vec::new();

        while let Expr::Apply { lhs, rhs } = callee {
            args.push(rhs);
            callee = lhs;
        }

        (callee, args.into_iter().rev().collect())
    }

    pub fn unlambda(&self) -> (Vec<&Identifier>, &Expr) {
        let mut params: Vec<&Identifier> = Vec::new();
        let mut body: &Expr = self;

        while let Expr::Lambda { param, body: next } = body {
            params.push(param);
            body = next;
        }

        (params, body)
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use crate::expr;

    #[test]
    fn test_unapply() {
        let expr = expr::a(expr::a("w", "x"), expr::a("y", "z"));

        let (callee, args) = expr.unapply();

        assert_eq!(callee, &expr::v("w"));
        assert_eq!(args, vec![&expr::v("x"), &expr::a("y", "z")]);
    }
}
