use super::Expr;

impl Expr {
    pub fn unapply(&self) -> (&Expr, ArgsIter) {
        let mut callee: &Expr = self;
        let mut args: Vec<&Expr> = Vec::new();

        while let Expr::Apply { lhs, rhs } = callee {
            args.push(rhs);
            callee = lhs;
        }

        (callee, ArgsIter { args })
    }
}

// ========================================================================== //

pub struct ArgsIter<'a> {
    args: Vec<&'a Expr>,
}

impl<'a> Iterator for ArgsIter<'a> {
    type Item = &'a Expr;

    fn next(&mut self) -> Option<Self::Item> {
        self.args.pop()
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use crate::expr;

    #[test]
    fn test_unapply() {
        let expr = expr::a(expr::a("w", "x"), expr::a("y", "z"));

        let (callee, mut args) = expr.unapply();

        assert_eq!(callee, &expr::v("w"));
        assert_eq!(args.next(), Some(&expr::v("x")));
        assert_eq!(args.next(), Some(&expr::a("y", "z")));
        assert_eq!(args.next(), None);
    }
}
