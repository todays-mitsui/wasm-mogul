use super::Expr;

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
}

// // ========================================================================== //

// pub struct ArgsIter<'a> {
//     args: Vec<&'a Expr>,
// }

// impl<'a> Iterator for ArgsIter<'a> {
//     type Item = &'a Expr;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.args.pop()
//     }
// }

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
