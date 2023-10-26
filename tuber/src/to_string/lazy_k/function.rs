use crate::func::Func;
use crate::style::LazyKStyle;
use regex::Regex;
use std::fmt::Display;

impl Display for LazyKStyle<'_, Func> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func: &Func = self.0;
        let mut tokens = tokens(func);
        write!(
            f,
            "{}{} = {}",
            "`".to_string().repeat(func.arity()),
            to_string(&mut tokens),
            func.body()
        )
    }
}

impl Display for LazyKStyle<'_, &Func> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func: &Func = self.0;
        let mut tokens = tokens(func);
        write!(
            f,
            "{}{} = {}",
            "`".to_string().repeat(func.arity()),
            to_string(&mut tokens),
            func.body()
        )
    }
}

fn tokens<'a>(f: &'a Func) -> Vec<Token<'a>> {
    let mut tokens = f
        .params()
        .iter()
        .map(|i| {
            let label = i.as_str();
            if is_upper_ident(label) {
                Token::UpperIdent(label)
            } else {
                Token::LowerIdent(label)
            }
        })
        .collect::<Vec<_>>();

    let label = f.name();
    if is_upper_ident(label) {
        tokens.insert(0, Token::UpperIdent(label));
    } else {
        tokens.insert(0, Token::LowerIdent(label));
    }

    tokens
}

fn to_string(tokens: &mut Vec<Token>) -> String {
    tokens.reverse();
    let mut str = String::new();
    while tokens.len() > 0 {
        match tokens.len() {
            1 => {
                let t = tokens.pop().unwrap();
                str.push_str(&format!("{}", t));
            }

            _ => {
                let t1 = tokens.pop().unwrap();
                let t2 = tokens.pop().unwrap();

                match (t1, &t2) {
                    (Token::UpperIdent(ident1), Token::UpperIdent(_)) => {
                        str.push_str(&format!("{} ", ident1));
                    }
                    (t1, _) => {
                        str.push_str(&format!("{}", t1));
                    }
                }

                tokens.push(t2);
            }
        }
    }
    str
}

fn is_upper_ident(s: &str) -> bool {
    let regex_upper_ident: Regex = Regex::new(r"^[A-Z0-9_]+$").unwrap();
    regex_upper_ident.is_match(s)
}

// ========================================================================== //

#[derive(Debug, PartialEq)]
enum Token<'a> {
    UpperIdent(&'a str),
    LowerIdent(&'a str),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::UpperIdent(i) => write!(f, "{}", i),
            Token::LowerIdent(i) => write!(f, "{}", i),
        }
    }
}

// ========================================================================== //
#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::func;

    #[test]
    fn test_to_string() {
        let f = func::new("f", vec!["x", "y"], expr::a("x", "y"));
        assert_eq!(LazyKStyle(&f).to_string(), "``fxy = `xy");

        let f = func::new("F", vec!["X", "Y"], expr::a("X", "Y"));
        assert_eq!(LazyKStyle(&f).to_string(), "``F X Y = `X Y");

        let f = func::new("F", vec!["x", "Y"], expr::a("x", "Y"));
        assert_eq!(LazyKStyle(&f).to_string(), "``FxY = `xY");
    }
}
