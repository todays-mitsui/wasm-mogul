use crate::context::Context;
use crate::expr::FreeVars;
use crate::expr::{self, Expr, Identifier};

pub fn unlambda(context: &Context, expr: Expr, iota: &Identifier) -> Expr {
    match expr {
        Expr::Variable(ref id) => {
            if iota == id {
                expr
            } else {
                match context.get(id) {
                    Some(func) => unlambda(context, func.to_owned().into(), iota),
                    None => expr,
                }
            }
        }
        Expr::Symbol(_) => expr,
        Expr::Apply { lhs, rhs } => {
            expr::a(unlambda(context, *lhs, iota), unlambda(context, *rhs, iota))
        }
        Expr::Lambda { param, body } => unlambda(context, unlambda_(*body, &param, iota), iota),
    }
}

fn unlambda_(expr: Expr, param: &Identifier, iota: &Identifier) -> Expr {
    match expr {
        Expr::Variable(id) if &id == param => {
            let iota = expr::v(iota.to_owned());
            let i = expr::a(iota.clone(), iota);
            i
        }
        Expr::Variable(_) | Expr::Symbol(_) => {
            let iota = expr::v(iota.to_owned());
            let k = expr::a(
                iota.clone(),
                expr::a(iota.clone(), expr::a(iota.clone(), iota)),
            );
            expr::a(k, expr)
        }
        Expr::Apply { .. } if !FreeVars::from(&expr).contains(param) => {
            let iota = expr::v(iota.to_owned());
            let k = expr::a(
                iota.clone(),
                expr::a(iota.clone(), expr::a(iota.clone(), iota)),
            );
            expr::a(k, expr)
        }
        Expr::Apply { lhs, rhs } => match rhs.as_ref() {
            Expr::Variable(id) if id == param && !FreeVars::from(lhs.as_ref()).contains(param) => {
                *lhs
            }
            _ => {
                let iota_ = expr::v(iota.to_owned());
                let s = expr::a(
                    iota_.clone(),
                    expr::a(
                        iota_.clone(),
                        expr::a(iota_.clone(), expr::a(iota_.clone(), iota_)),
                    ),
                );
                expr::a(
                    expr::a(s, unlambda_(*lhs, param, iota)),
                    unlambda_(*rhs, param, iota),
                )
            }
        },
        Expr::Lambda { param: inner, body } => {
            unlambda_(unlambda_(*body, &inner, iota), param, iota)
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func;
    use rand::Rng;
    use std::str;

    fn setup() -> (Identifier, Context) {
        let iota: Identifier = {
            let mut rng = rand::thread_rng();
            let c = [rng.gen_range(b'a'..=b'z')];
            let s = str::from_utf8(&c).unwrap();
            Identifier::from(s)
        };

        let context: Context = {
            Context::from(vec![
                func::new(
                    iota.to_owned(),
                    vec!["f"],
                    expr::a(
                        expr::a(
                            expr::v("f"),
                            expr::l(
                                "x",
                                expr::l(
                                    "y",
                                    expr::l("z", expr::a(expr::a("x", "z"), expr::a("y", "z"))),
                                ),
                            ),
                        ),
                        expr::l("x", expr::l("y", "x")),
                    ),
                ),
                func::new("TRUE", vec!["THEN", "ELSE"], expr::v("THEN")),
                func::new("FALSE", vec!["THEN", "ELSE"], expr::v("ELSE")),
                func::new(
                    "NOT",
                    vec!["x"],
                    expr::a(expr::a(expr::v("x"), expr::v("FALSE")), expr::v("TRUE")),
                ),
            ])
        };

        (iota, context)
    }

    #[test]
    fn test_unlambda() {
        let (iota, context) = setup();
        let iota_ = expr::v(iota.to_owned());

        let i = expr::a(iota_.clone(), iota_.clone());
        let k = expr::a(
            iota_.clone(),
            expr::a(iota_.clone(), expr::a(iota_.clone(), iota_.clone())),
        );
        let s = expr::a(
            iota_.clone(),
            expr::a(
                iota_.clone(),
                expr::a(iota_.clone(), expr::a(iota_.clone(), iota_.clone())),
            ),
        );

        // x == x
        let source = expr::v("x");
        let expected = expr::v("x");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // :x = :x
        let source = expr::s("x");
        let expected = expr::s("x");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // `xy == `xy
        let source = expr::a("x", "y");
        let expected = expr::a("x", "y");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.x == i
        let source = expr::l("x", "x");
        let expected = i.clone();
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.:x == `k:x
        let source = expr::l("x", ":x");
        let expected = expr::a(k.clone(), ":x");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.y == `ky
        let source = expr::l("x", "y");
        let expected = expr::a(k.clone(), "y");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.:y == `k:y
        let source = expr::l("x", ":y");
        let expected = expr::a(k.clone(), ":y");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.`yx == y
        let source = expr::l("x", expr::a("y", "x"));
        let expected = expr::v("y");
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.`y:x == `k`y:x
        let source = expr::l("x", expr::a("y", ":x"));
        let expected = expr::a(k.clone(), expr::a("y", ":x"));
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.`xy == ``si`ky
        let source = expr::l("x", expr::a("x", "y"));
        let expected = expr::a(expr::a(s.clone(), i.clone()), expr::a(k.clone(), "y"));
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.`:xy == `k`:xy
        let source = expr::l("x", expr::a(":x", "y"));
        let expected = expr::a(k.clone(), expr::a(":x", "y"));
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.`yz == `k`yz
        let source = expr::l("x", expr::a("y", "z"));
        let expected = expr::a(k.clone(), expr::a("y", "z"));
        assert_eq!(unlambda(&context, source, &iota), expected);

        // ^x.^y.`xy == i
        let source = expr::l("x", expr::l("y", expr::a("x", "y")));
        let expected = i.clone();
        assert_eq!(unlambda(&context, source, &iota), expected);

        // TRUE => ^THEN.^ELSE.THEN => k
        let source = expr::v("TRUE");
        let expected = k.clone();
        assert_eq!(unlambda(&context, source, &iota), expected);

        // FALSE => ^THEN.^ELSE.ELSE => `ki
        let source = expr::v("FALSE");
        let expected = expr::a(k.clone(), i.clone());
        assert_eq!(unlambda(&context, source, &iota), expected);

        // NOT => ^x.``xFALSE TRUE => ``s``si`kFALSE`kTRUE
        //   => ``s``si`k^THEN.^ELSE.ELSE`kTRUE
        //   => ``s``si`k`ki`kTRUE
        //   => ``s``si`k`ki`k^THEN.^ELSE.THEN
        //   => ``s``si`k`ki`kk
        let source = expr::v("NOT");
        let expected = expr::a(
            expr::a(
                s.clone(),
                expr::a(
                    expr::a(s.clone(), i.clone()),
                    expr::a(k.clone(), expr::a(k.clone(), i.clone())),
                ),
            ),
            expr::a(k.clone(), k.clone()),
        );
        assert_eq!(unlambda(&context, source, &iota), expected);
    }
}
