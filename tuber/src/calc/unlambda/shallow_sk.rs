use crate::expr::FreeVars;
use crate::expr::{self, Expr, Identifier};

pub fn unlambda(expr: Expr, sk: &[&Identifier; 2]) -> Expr {
    match expr {
        Expr::Variable(_) => expr,
        Expr::Symbol(_) => expr,
        Expr::Apply { lhs, rhs } => expr::a(unlambda(*lhs, sk), unlambda(*rhs, sk)),
        Expr::Lambda { param, body } => unlambda_(*body, &param, sk),
    }
}

fn unlambda_(expr: Expr, param: &Identifier, sk: &[&Identifier; 2]) -> Expr {
    let s: &Identifier = sk[0];
    let k: &Identifier = sk[1];

    match expr {
        Expr::Variable(id) if &id == param => expr::a(
            expr::a(expr::v(s.to_owned()), expr::v(k.to_owned())),
            expr::v(k.to_owned()),
        ),
        Expr::Variable(_) => expr::a(expr::v(k.to_owned()), expr),
        Expr::Symbol(_) => expr::a(expr::v(k.to_owned()), expr),
        Expr::Apply { .. } if !FreeVars::from(&expr).contains(param) => {
            expr::a(expr::v(k.to_owned()), expr)
        }
        Expr::Apply { lhs, rhs } => match rhs.as_ref() {
            Expr::Variable(id) if id == param && !FreeVars::from(lhs.as_ref()).contains(param) => {
                *lhs
            }
            _ => expr::a(
                expr::a(expr::v(s.to_owned()), unlambda_(*lhs, param, sk)),
                unlambda_(*rhs, param, sk),
            ),
        },
        Expr::Lambda { param: inner, body } => unlambda_(unlambda_(*body, &inner, sk), param, sk),
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::str;

    fn setup() -> [Identifier; 2] {
        let mut rng = rand::thread_rng();

        let mut ids: Vec<Identifier> = Vec::new();
        while ids.len() < 2 {
            let c = [rng.gen_range(b'a'..=b'z')];
            let s = str::from_utf8(&c).unwrap();
            let id = Identifier::from(s);
            if !ids.contains(&id) {
                ids.push(id);
            }
        }

        ids.try_into().unwrap()
    }

    #[test]
    fn test_unlambda() {
        let ids = setup();
        let sk: [&Identifier; 2] = ids.iter().collect::<Vec<_>>().try_into().unwrap();
        let s: &Identifier = sk[0];
        let k: &Identifier = sk[1];
        let (s, k) = (expr::v(s.to_owned()), expr::v(k.to_owned()));

        let skk = expr::a(expr::a(s.clone(), k.clone()), k.clone());

        // x == x
        let source = expr::v("x");
        let expected = expr::v("x");
        assert_eq!(unlambda(source, &sk), expected);

        // :x = :x
        let source = expr::s("x");
        let expected = expr::s("x");
        assert_eq!(unlambda(source, &sk), expected);

        // `xy == `xy
        let source = expr::a("x", "y");
        let expected = expr::a("x", "y");
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.x == ``skk
        let source = expr::l("x", "x");
        let expected = skk.clone();
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.:x == `k:x
        let source = expr::l("x", ":x");
        let expected = expr::a(k.clone(), ":x");
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.y == `ky
        let source = expr::l("x", "y");
        let expected = expr::a(k.clone(), "y");
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.:y == `k:y
        let source = expr::l("x", ":y");
        let expected = expr::a(k.clone(), ":y");
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.`yx == y
        let source = expr::l("x", expr::a("y", "x"));
        let expected = expr::v("y");
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.`y:x == `k`y:x
        let source = expr::l("x", expr::a("y", ":x"));
        let expected = expr::a(k.clone(), expr::a("y", ":x"));
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.`xy == ``s``skk`ky
        let source = expr::l("x", expr::a("x", "y"));
        let expected = expr::a(expr::a(s.clone(), skk.clone()), expr::a(k.clone(), "y"));
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.`:xy == `k`:xy
        let source = expr::l("x", expr::a(":x", "y"));
        let expected = expr::a(k.clone(), expr::a(":x", "y"));
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.`yz == `k`yz
        let source = expr::l("x", expr::a("y", "z"));
        let expected = expr::a(k.clone(), expr::a("y", "z"));
        assert_eq!(unlambda(source, &sk), expected);

        // ^x.^y.`xy == ``skk
        let source = expr::l("x", expr::l("y", expr::a("x", "y")));
        let expected = skk.clone();
        assert_eq!(unlambda(source, &sk), expected);
    }
}
