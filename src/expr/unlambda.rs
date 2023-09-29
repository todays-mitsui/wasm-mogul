use super::free_vars::FreeVars;
use crate::expr::{self, Expr, Identifier};

// TODO: ski 以外の出現を許さないバージョンの unlambda も必要
// TODO: sk 以外の出現を許さないバージョンの unlambda も必要
// TODO: ι 以外の出現を許さないバージョンの unlambda も必要

pub fn unlambda(expr: Expr) -> Expr {
    match expr {
        Expr::Variable(_) => expr,
        Expr::Symbol(_) => expr,
        Expr::Apply { lhs, rhs } => expr::a(unlambda(*lhs), unlambda(*rhs)),
        Expr::Lambda { param, body } => unlambda_(*body, &param),
    }
}

fn unlambda_(expr: Expr, param: &Identifier) -> Expr {
    match expr {
        Expr::Variable(id) if &id == param => expr::v("i"),
        Expr::Variable(id) => expr::a("k", expr::v(id)),
        Expr::Symbol(_) => expr::a("k", expr),
        Expr::Apply { .. } if !FreeVars::from(&expr).contains(param) => expr::a("k", expr),
        Expr::Apply { lhs, rhs } => match rhs.as_ref() {
            Expr::Variable(id) if id == param && !FreeVars::from(lhs.as_ref()).contains(param) => {
                *lhs
            }
            _ => expr::a(expr::a("s", unlambda_(*lhs, param)), unlambda_(*rhs, param)),
        },
        Expr::Lambda { param: inner, body } => unlambda_(unlambda_(*body, &inner), param),
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlambda() {
        // x == x
        let source = expr::v("x");
        let expected = expr::v("x");
        assert_eq!(unlambda(source), expected);

        // :x = :x
        let source = expr::s("x");
        let expected = expr::s("x");
        assert_eq!(unlambda(source), expected);

        // `xy == `xy
        let source = expr::a("x", "y");
        let expected = expr::a("x", "y");
        assert_eq!(unlambda(source), expected);

        // ^x.x == i
        let source = expr::l("x", "x");
        let expected = expr::v("i");
        assert_eq!(unlambda(source), expected);

        // ^x.:x == `k:x
        let source = expr::l("x", ":x");
        let expected = expr::a("k", ":x");
        assert_eq!(unlambda(source), expected);

        // ^x.y == `ky
        let source = expr::l("x", "y");
        let expected = expr::a("k", "y");
        assert_eq!(unlambda(source), expected);

        // ^x.:y == `k:y
        let source = expr::l("x", ":y");
        let expected = expr::a("k", ":y");
        assert_eq!(unlambda(source), expected);

        // ^x.`yx == y
        let source = expr::l("x", expr::a("y", "x"));
        let expected = expr::v("y");
        assert_eq!(unlambda(source), expected);

        // ^x.`y:x == `k`y:x
        let source = expr::l("x", expr::a("y", ":x"));
        let expected = expr::a("k", expr::a("y", ":x"));
        assert_eq!(unlambda(source), expected);

        // ^x.`xy == ``si`ky
        let source = expr::l("x", expr::a("x", "y"));
        let expected = expr::a(expr::a("s", "i"), expr::a("k", "y"));
        assert_eq!(unlambda(source), expected);

        // ^x.`:xy == `k`:xy
        let source = expr::l("x", expr::a(":x", "y"));
        let expected = expr::a("k", expr::a(":x", "y"));
        assert_eq!(unlambda(source), expected);

        // ^x.`yz == `k`yz
        let source = expr::l("x", expr::a("y", "z"));
        let expected = expr::a("k", expr::a("y", "z"));
        assert_eq!(unlambda(source), expected);

        // ^x.^y.`xy == i
        let source = expr::l("x", expr::l("y", expr::a("x", "y")));
        let expected = expr::v("i");
        assert_eq!(unlambda(source), expected);
    }
}
