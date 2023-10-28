use crate::expr::Expr;
use regex::Regex;
use std::fmt::Display;

pub fn to_string(expr: &Expr) -> String {
    tokens_to_string(&mut tokens(expr))
}

// ========================================================================== //

fn tokens<'a>(expr: &'a Expr) -> Vec<Token<'a>> {
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

        Expr::Apply { lhs, rhs } => {
            let mut lhs = tokens(lhs);
            let mut rhs = tokens(rhs);
            rhs.append(&mut lhs);
            rhs.push(Token::Apply);
            rhs
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
    }
}

fn tokens_to_string(tokens: &mut Vec<Token>) -> String {
    let mut str = String::new();
    while tokens.len() > 0 {
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
