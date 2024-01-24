use super::super::compact::Compact;
use super::super::formed::Formed;
use super::super::tag::Tag;
use crate::expr::Expr;
use crate::expr::Path;

pub fn format(expr: &Expr, splits: &[Path]) -> Formed {
    Compact::from(expr).reform(splits).into()
}

impl From<Compact<'_>> for Formed {
    fn from(compact: Compact<'_>) -> Self {
        match compact {
            Compact::Variable { label, tag } => {
                let mut expr = String::new();
                let mut mapping: Vec<Tag> = Vec::new();

                let str = label;
                let mut tags = vec![tag; str.chars().count()];

                expr.push_str(str);
                mapping.append(&mut tags);

                Formed { expr, mapping }
            }

            Compact::Symbol { label, tag } => {
                let mut expr = String::new();
                let mut mapping: Vec<Tag> = Vec::new();

                let str = label;
                let mut tags = vec![tag; 1 + label.chars().count()];

                expr.push_str(":");
                expr.push_str(str);
                mapping.append(&mut tags);

                Formed { expr, mapping }
            }

            Compact::Lambda { params, body, tag } => {
                let mut expr = String::new();
                let mut mapping: Vec<Tag> = Vec::new();

                if params.len() == 1 {
                    expr = expr + &params[0];
                } else {
                    expr = expr + "(" + &params.join(", ") + ")"
                };
                expr = expr + " => ";
                let body_str = Formed::from(*body).expr;
                expr = expr + &body_str;

                mapping.append(&mut vec![tag.push(0); expr.chars().count()]);

                Formed { expr, mapping }
            }

            Compact::Apply { callee, args, tag } => {
                let mut expr = String::new();
                let mut mapping: Vec<Tag> = Vec::new();

                match *callee {
                    Compact::Lambda { .. } => {
                        let mut formed = Formed::from(*callee);
                        expr = expr + "(" + &formed.expr + ")";
                        mapping.push(tag.clone());
                        mapping.append(&mut formed.mapping);
                        mapping.push(tag.clone());
                    }
                    _ => {
                        let mut formed = Formed::from(*callee);
                        expr = expr + &formed.expr;
                        mapping.append(&mut formed.mapping);
                    }
                }

                expr = expr + "(";
                mapping.push(tag.clone());

                let len = args.len();
                for (index, arg) in args.into_iter().enumerate() {
                    let mut formed = Formed::from(arg);
                    expr = expr + &formed.expr;
                    mapping.append(&mut formed.mapping);

                    if index < len - 1 {
                        expr = expr + ", ";
                        mapping.append(&mut vec![tag.clone(); 2]);
                    }
                }

                expr = expr + ")";
                mapping.push(tag.clone());

                Formed { expr, mapping }
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
    fn test_format_1() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");

        let formed = format(&expr, &vec![]);

        println!("{:#?}", formed);

        assert_eq!(formed.expr, "f(w, x, y, z)");
        assert_eq!(
            formed.mapping,
            vec![
                /* f */ Tag::from(vec![0]),
                /* ( */ Tag::from(vec![4]),
                /* w */ Tag::from(vec![1, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* x */ Tag::from(vec![2, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* y */ Tag::from(vec![3, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* z */ Tag::from(vec![4, 0]),
                /* ) */ Tag::from(vec![4]),
            ]
        );
    }

    #[test]
    fn test_format_2() {
        let expr = expr::a(expr::a(expr::a(expr::a("f", "w"), "x"), "y"), "z");

        let formed = format(&expr, &vec![Path::Callee(1), Path::Callee(3)]);

        println!("{:?}", formed.expr);
        println!("{:#?}", formed);

        assert_eq!(formed.expr, "f(w)(x, y)(z)");
        assert_eq!(
            formed.mapping,
            vec![
                /* f */ Tag::from(vec![0]),
                /* ( */ Tag::from(vec![1]),
                /* w */ Tag::from(vec![1, 0]),
                /* ) */ Tag::from(vec![1]),
                /* ( */ Tag::from(vec![3]),
                /* x */ Tag::from(vec![2, 0]),
                /* , */ Tag::from(vec![3]),
                /*   */ Tag::from(vec![3]),
                /* y */ Tag::from(vec![3, 0]),
                /* ) */ Tag::from(vec![3]),
                /* ( */ Tag::from(vec![4]),
                /* z */ Tag::from(vec![4, 0]),
                /* ) */ Tag::from(vec![4]),
            ]
        );
    }

    #[test]
    fn test_format_3() {
        let expr = expr::a(
            expr::a(
                expr::a(expr::a("a", "b"), "c"),
                expr::a(expr::a("d", "e"), "f"),
            ),
            expr::a("g", "h"),
        );

        let formed = format(&expr, &vec![]);

        println!("{:?}", formed.expr);
        println!("{:#?}", formed);

        assert_eq!(formed.expr, "a(b, c, d(e, f), g(h))");
        assert_eq!(
            formed.mapping,
            vec![
                /* a */ Tag::from(vec![0]),
                /* ( */ Tag::from(vec![4]),
                /* b */ Tag::from(vec![1, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* c */ Tag::from(vec![2, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* d */ Tag::from(vec![3, 0]),
                /* ( */ Tag::from(vec![3, 2]),
                /* e */ Tag::from(vec![3, 1, 0]),
                /* , */ Tag::from(vec![3, 2]),
                /*   */ Tag::from(vec![3, 2]),
                /* f */ Tag::from(vec![3, 2, 0]),
                /* ) */ Tag::from(vec![3, 2]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* g */ Tag::from(vec![4, 0]),
                /* ( */ Tag::from(vec![4, 1]),
                /* h */ Tag::from(vec![4, 1, 0]),
                /* ) */ Tag::from(vec![4, 1]),
                /* ) */ Tag::from(vec![4]),
            ]
        );
    }

    #[test]
    fn test_format_4() {
        let expr = expr::a(
            expr::a(
                expr::a(expr::a("a", "b"), "c"),
                expr::a(expr::a("d", "e"), "f"),
            ),
            expr::a("g", "h"),
        );

        let formed = format(&expr, &vec![Path::Arg(2, Box::new(Path::Callee(1)))]);

        println!("{:?}", formed.expr);
        println!("{:#?}", formed);

        assert_eq!(formed.expr, "a(b, c, d(e)(f), g(h))");
        assert_eq!(
            formed.mapping,
            vec![
                /* a */ Tag::from(vec![0]),
                /* ( */ Tag::from(vec![4]),
                /* b */ Tag::from(vec![1, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* c */ Tag::from(vec![2, 0]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* d */ Tag::from(vec![3, 0]),
                /* ( */ Tag::from(vec![3, 1]),
                /* e */ Tag::from(vec![3, 1, 0]),
                /* ) */ Tag::from(vec![3, 1]),
                /* ( */ Tag::from(vec![3, 2]),
                /* f */ Tag::from(vec![3, 2, 0]),
                /* ) */ Tag::from(vec![3, 2]),
                /* , */ Tag::from(vec![4]),
                /*   */ Tag::from(vec![4]),
                /* g */ Tag::from(vec![4, 0]),
                /* ( */ Tag::from(vec![4, 1]),
                /* h */ Tag::from(vec![4, 1, 0]),
                /* ) */ Tag::from(vec![4, 1]),
                /* ) */ Tag::from(vec![4]),
            ]
        );
    }
}
