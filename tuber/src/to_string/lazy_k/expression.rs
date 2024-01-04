use crate::expr::Expr;
use crate::expr::Path;
use regex::Regex;
use std::fmt::Display;

pub fn to_string(expr: &Expr) -> String {
    tokens_to_string(&mut tokens(expr))
}

// ========================================================================== //

fn tokens_with_range<'a>(expr: &'a Expr, path: &Path) -> (Vec<Token<'a>>, Range) {
    let mut callee: &Expr = expr;
    let mut args: Vec<&Expr> = Vec::new();
    while let Expr::Apply { lhs, rhs } = callee {
        callee = lhs;
        args.push(rhs);
    }

    let mut callee_tokens = tokens(callee);

    let (mut after_tokens, mut arg_tokens, mut before_tokens, range) = match path {
        Path::Arg(index, next) => {
            let (after, arg, before) = rev_partition(&args, *index);

            let after_tokens = after.iter().flat_map(|e| tokens(*e)).collect::<Vec<_>>();
            let (arg_tokens, range) = tokens_with_range(arg, next);
            let before_tokens = before.iter().flat_map(|e| tokens(*e)).collect::<Vec<_>>();

            let offset = args.len() // バッククォートの数
                + callee_tokens.len()
                + before_tokens.len(); // index よりも左の引数

            let range = Range {
                start: offset + range.start,
                end: offset + range.end,
            };

            (after_tokens, arg_tokens, before_tokens, range)
        }

        Path::Callee(arity) => {
            let (after, arg, before) = rev_partition(&args, *arity);

            let after_tokens = after.iter().flat_map(|e| tokens(*e)).collect::<Vec<_>>();
            let arg_tokens = tokens(arg);
            let before_tokens = before.iter().flat_map(|e| tokens(*e)).collect::<Vec<_>>();

            let num_back_quotes = args.len(); // バッククォートの数
            let range = Range {
                start: num_back_quotes - arity, // 全てのバッククォートから着目している関数適用に属する分を差し引く
                end: num_back_quotes + callee_tokens.len() + before_tokens.len() + arg_tokens.len(),
            };

            (after_tokens, arg_tokens, before_tokens, range)
        }
    };

    let mut tokens = Vec::new();
    tokens.append(&mut after_tokens);
    tokens.append(&mut arg_tokens);
    tokens.append(&mut before_tokens);
    tokens.append(&mut callee_tokens);
    for _ in 0..args.len() {
        tokens.push(Token::Apply);
    }

    return (tokens, range);
}

fn tokens(expr: &Expr) -> Vec<Token<'_>> {
    match expr {
        Expr::Variable(i) => {
            let label = i.as_str();
            if is_upper_ident(label) {
                vec![Token::UpperIdent(Ident::Variable(label))]
            } else {
                vec![Token::LowerIdent(Ident::Variable(label))]
            }
        }

        Expr::Symbol(i) => {
            let label = i.as_str();
            if is_upper_ident(label) {
                vec![Token::UpperIdent(Ident::Symbol(label))]
            } else {
                vec![Token::LowerIdent(Ident::Symbol(label))]
            }
        }

        Expr::Lambda { param, body } => {
            let mut body = tokens(body);
            body.push(Token::Dot);
            let label = param.as_str();
            if is_upper_ident(label) {
                body.push(Token::UpperIdent(Ident::Variable(label)));
            } else {
                body.push(Token::LowerIdent(Ident::Variable(label)));
            }
            body.push(Token::Lambda);
            body
        }

        Expr::Apply { .. } => unreachable!("Apply should be handled in tokens_with_range"),
    }
}

fn tokens_to_string(tokens: &mut Vec<Token>) -> String {
    let mut str = String::new();
    while !tokens.is_empty() {
        match tokens.len() {
            1 => {
                let t = tokens.pop().unwrap();
                str.push_str(&format!("{}", t))
            }
            _ => {
                let t1 = tokens.pop().unwrap();
                let t2 = tokens.pop().unwrap();
                match (t1, &t2) {
                    (Token::UpperIdent(ident1), Token::UpperIdent(Ident::Variable(_))) => {
                        str.push_str(format!("{} ", ident1).as_str());
                        tokens.push(t2);
                    }
                    (t1, _) => {
                        str.push_str(format!("{}", t1).as_str());
                        tokens.push(t2);
                    }
                }
            }
        }
    }
    str
}

fn is_upper_ident(s: &str) -> bool {
    let regex_upper_ident: Regex = Regex::new(r"\A[A-Z0-9_]+\z").unwrap();
    regex_upper_ident.is_match(s)
}

// ========================================================================== //

fn range(expr: &Expr, path: &Path) -> Range {
    let mut callee: &Expr = expr;
    let mut args: Vec<&Expr> = Vec::new();
    while let Expr::Apply { lhs, rhs } = callee {
        callee = lhs;
        args.push(rhs);
    }

    match path {
        Path::Arg(index, next) => {
            let num_callee_tokens = tokens(callee).len();

            let (_, arg, before) = rev_partition(&args, *index);

            let num_before_tokens = before.iter().map(|e| tokens(*e).len()).sum::<usize>();
            let range = range(arg, next);

            return Range {
                start: args.len() + num_callee_tokens + num_before_tokens + range.start,
                end: args.len() + num_callee_tokens + num_before_tokens + range.end,
            };
        }

        Path::Callee(arity) => {
            let num_callee_tokens = tokens(callee).len();
            let num_args_tokens = args
                .iter()
                .rev()
                .take(*arity)
                .map(|e| tokens(*e).len())
                .sum::<usize>();

            let start = args.len() - arity;
            let end = args.len() + num_callee_tokens + num_args_tokens;
            return Range { start, end };
        }
    }
}

fn rev_partition<T: Clone + Copy>(args: &Vec<T>, index: usize) -> (Vec<T>, T, Vec<T>) {
    let len = args.len();
    (
        args[0..len - index - 1].to_vec(),
        args[len - index - 1],
        args[len - index..].to_vec(),
    )
}

#[test]
fn test_rev_partition() {
    let args = vec![5, 4, 3, 2, 1];

    assert_eq!(rev_partition(&args, 0), (vec![5, 4, 3, 2], 1, vec![]));
    assert_eq!(rev_partition(&args, 1), (vec![5, 4, 3], 2, vec![1]));
    assert_eq!(rev_partition(&args, 2), (vec![5, 4], 3, vec![2, 1]));
    assert_eq!(rev_partition(&args, 3), (vec![5], 4, vec![3, 2, 1]));
    assert_eq!(rev_partition(&args, 4), (vec![], 5, vec![4, 3, 2, 1]));
}

#[test]
fn test_range() {
    use crate::expr;

    // expr = ````abc``def`gh
    let expr = expr::a(
        expr::a(
            expr::a(expr::a(expr::v("a"), expr::v("b")), expr::v("c")),
            expr::a(expr::a(expr::v("d"), expr::v("e")), expr::s("f")),
        ),
        expr::a(expr::s("g"), expr::s("h")),
    );

    // expr = ````abc``def`gh
    //            ^
    let path = Path::Callee(0);
    assert_eq!(range(&expr, &path), Range { start: 4, end: 5 });

    // expr = ````abc``def`gh
    //           ^^^
    let path = Path::Callee(1);
    assert_eq!(range(&expr, &path), Range { start: 3, end: 6 });

    // expr = ````abc``def`gh
    //          ^^^^^
    let path = Path::Callee(2);
    assert_eq!(range(&expr, &path), Range { start: 2, end: 7 });

    // expr = ````abc``def`gh
    //         ^^^^^^^^^^^
    let path = Path::Callee(3);
    assert_eq!(range(&expr, &path), Range { start: 1, end: 12 });

    // expr = ````abc``def`gh
    //        ^^^^^^^^^^^^^^^
    let path = Path::Callee(4);
    assert_eq!(range(&expr, &path), Range { start: 0, end: 15 });

    // expr = ````abc``def`gh
    //             ^
    let path = Path::Arg(0, Box::new(Path::Callee(0)));
    assert_eq!(range(&expr, &path), Range { start: 5, end: 6 });

    // expr = ````abc``def`gh
    //              ^
    let path = Path::Arg(1, Box::new(Path::Callee(0)));
    assert_eq!(range(&expr, &path), Range { start: 6, end: 7 });

    // expr = ````abc``def`gh
    //                 ^
    let path = Path::Arg(2, Box::new(Path::Callee(0)));
    assert_eq!(range(&expr, &path), Range { start: 9, end: 10 });

    // expr = ````abc``def`gh
    //                ^^^
    let path = Path::Arg(2, Box::new(Path::Callee(1)));
    assert_eq!(range(&expr, &path), Range { start: 8, end: 11 });

    // expr = ````abc``def`gh
    //               ^^^^^
    let path = Path::Arg(2, Box::new(Path::Callee(2)));
    assert_eq!(range(&expr, &path), Range { start: 7, end: 12 });

    // expr = ````abc``def`gh
    //                     ^
    let path = Path::Arg(3, Box::new(Path::Callee(0)));
    assert_eq!(range(&expr, &path), Range { start: 13, end: 14 });

    // expr = ````abc``def`gh
    //                    ^^^
    let path = Path::Arg(3, Box::new(Path::Callee(1)));
    assert_eq!(range(&expr, &path), Range { start: 12, end: 15 });
}

#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
enum Token<'a> {
    UpperIdent(Ident<'a>),
    LowerIdent(Ident<'a>),
    Apply,
    Lambda,
    Dot,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::UpperIdent(i) | Token::LowerIdent(i) => write!(f, "{}", i),
            Token::Apply => write!(f, "`"),
            Token::Lambda => write!(f, "λ"),
            Token::Dot => write!(f, "."),
        }
    }
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
enum Ident<'a> {
    Variable(&'a str),
    Symbol(&'a str),
}

impl Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ident::Variable(label) => write!(f, "{}", label),
            Ident::Symbol(label) => write!(f, ":{}", label),
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
        assert_eq!(to_string(&expr::l("x", "y")), "λx.y");
        assert_eq!(to_string(&expr::l("x", expr::a("y", "z"))), "λx.`yz");
        assert_eq!(to_string(&expr::l("X", expr::a("y", "z"))), "λX.`yz");
        assert_eq!(to_string(&expr::a("x", "y")), "`xy");
        assert_eq!(to_string(&expr::a("x", "Y")), "`xY");
        assert_eq!(to_string(&expr::a("X", "y")), "`Xy");
        assert_eq!(to_string(&expr::a("X", "Y")), "`X Y");
        assert_eq!(to_string(&expr::a("X", ":Y")), "`X:Y");
        assert_eq!(to_string(&expr::a(":X", "Y")), "`:X Y");
        assert_eq!(to_string(&expr::a(":X", ":Y")), "`:X:Y");
    }

    #[test]
    fn test_tokens() {
        assert_eq!(
            tokens(&expr::v("x")),
            vec![Token::LowerIdent(Ident::Variable("x"))]
        );
        assert_eq!(
            tokens(&expr::v("FOO")),
            vec![Token::UpperIdent(Ident::Variable("FOO"))]
        );
        assert_eq!(
            tokens(&expr::s("a")),
            vec![Token::LowerIdent(Ident::Symbol("a"))]
        );
        assert_eq!(
            tokens(&expr::s("BAR")),
            vec![Token::UpperIdent(Ident::Symbol("BAR"))]
        );
        assert_eq!(
            tokens(&expr::a("x", "y")),
            vec![
                Token::LowerIdent(Ident::Variable("y")),
                Token::LowerIdent(Ident::Variable("x")),
                Token::Apply
            ]
        );
        assert_eq!(
            tokens(&expr::l("x", "y")),
            vec![
                Token::LowerIdent(Ident::Variable("y")),
                Token::Dot,
                Token::LowerIdent(Ident::Variable("x")),
                Token::Lambda
            ]
        );
    }
}
