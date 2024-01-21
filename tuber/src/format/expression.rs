use super::tag::Tag;
use crate::expr::Expr;

pub fn format(expr: &Expr) -> Formed {
    tokenize(expr, &Tag::new()).into()
}

pub struct Formed {
    pub expr: String,
    pub mapping: Vec<Tag>,
}

impl From<(Vec<Token<'_>>, Vec<Tag>)> for Formed {
    fn from((tokens, tags): (Vec<Token<'_>>, Vec<Tag>)) -> Self {
        assert!(tokens.len() == tags.len());

        let mut string: String = String::new();
        let mut mapping: Vec<Tag> = Vec::new();

        for index in 0..tokens.len() {
            let token: &Token<'_> = &tokens[index];
            let tag = &tags[index];

            let token_str = token.to_string();
            let mut token_tags = vec![tag.to_owned(); token_str.chars().count()];

            string.push_str(&token_str);
            mapping.append(&mut token_tags);

            if needs_space(token, tokens.get(index + 1)) {
                string.push(' ');
                mapping.push(Tag::new());
            }
        }

        assert!(string.chars().count() == mapping.len());

        Formed {
            expr: string,
            mapping,
        }
    }
}

fn needs_space(token: &Token, next_token: Option<&Token>) -> bool {
    if let Token::UpperIdent(_) = token {
        if let Some(Token::UpperIdent(_)) = next_token {
            return true;
        }
    }

    false
}

impl std::fmt::Debug for Formed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = Vec::new();
        for (index, char) in self.expr.chars().enumerate() {
            let tag = &self.mapping[index];
            lines.push(format!("{} : {:?}", char, tag));
        }
        write!(f, "{}", lines.join("\n"))
    }
}

// ========================================================================== //

fn tokenize<'a>(expr: &'a Expr, tag: &Tag) -> (Vec<Token<'a>>, Vec<Tag>) {
    match expr {
        Expr::Apply { .. } => {
            let (callee, args) = expr.unapply();
            assert!(!args.is_empty());

            let mut tokens: Vec<Token> = Vec::new();
            let mut tags: Vec<Tag> = Vec::new();

            // ============================================================== //

            for index in (0..args.len()).rev() {
                let mark_tag = tag.push(index + 1);
                tokens.push(Token::Apply);
                tags.push(mark_tag);
            }

            // ============================================================== //

            let (mut callee_tokens, mut callee_tags) = tokenize(callee, tag);

            tokens.append(&mut callee_tokens);
            tags.append(&mut callee_tags);

            // ============================================================== //

            for (index, arg) in args.into_iter().enumerate() {
                let arg_tag = tag.push(index + 1);
                let (mut arg_tokens, mut arg_tags) = tokenize(arg, &arg_tag);
                tokens.append(&mut arg_tokens);
                tags.append(&mut arg_tags);
            }

            // ============================================================== //

            (tokens, tags)
        }

        Expr::Variable(name) => {
            let ident = Ident::Variable(name.as_str());
            let token = if ident.is_lower() {
                Token::LowerIdent(ident)
            } else {
                Token::UpperIdent(ident)
            };
            let tag = tag.push(0);
            (vec![token], vec![tag])
        }

        Expr::Symbol(name) => {
            let ident = Ident::Symbol(name.as_str());
            let token = if ident.is_lower() {
                Token::LowerIdent(ident)
            } else {
                Token::UpperIdent(ident)
            };
            let tag = tag.push(0);
            (vec![token], vec![tag])
        }

        Expr::Lambda { param, body } => {
            let ident = Ident::Variable(param.as_str());
            let param_token = if ident.is_lower() {
                Token::LowerIdent(ident)
            } else {
                Token::UpperIdent(ident)
            };

            let (mut body_tokens, _) = tokenize(body, &Tag::new());

            let mut tokens = vec![Token::Lambda, param_token, Token::Dot];
            tokens.append(&mut body_tokens);

            let tags = vec![tag.push(0); tokens.len()];

            (tokens, tags)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token<'a> {
    UpperIdent(Ident<'a>),
    LowerIdent(Ident<'a>),
    Apply,
    Lambda,
    Dot,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::UpperIdent(ident) => write!(f, "{}", ident),
            Token::LowerIdent(ident) => write!(f, "{}", ident),
            Token::Apply => write!(f, "`"),
            Token::Lambda => write!(f, "λ"),
            Token::Dot => write!(f, "."),
        }
    }
}

// ========================================================================== //

#[derive(Clone, Debug, PartialEq)]
enum Ident<'a> {
    Variable(&'a str),
    Symbol(&'a str),
}

impl<'a> Ident<'a> {
    fn is_lower(&self) -> bool {
        match self {
            Ident::Variable(label) => Ident::<'a>::_is_lower(label),
            Ident::Symbol(label) => Ident::<'a>::_is_lower(label),
        }
    }

    fn _is_lower(str: &str) -> bool {
        str.len() == 1 && str.chars().next().unwrap().is_ascii_lowercase()
    }
}

impl std::fmt::Display for Ident<'_> {
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
    fn test_format_1() {
        let expr = expr::a(expr::a("w", "x"), expr::a("y", "z"));
        let formed = super::format(&expr);
        println!("{:?}", formed);
        assert_eq!(formed.expr, "``wx`yz");
        assert_eq!(
            formed.mapping,
            vec![
                /* ` */ Tag::from(vec![2]),
                /* ` */ Tag::from(vec![1]),
                /* w */ Tag::from(vec![0]),
                /* x */ Tag::from(vec![1, 0]),
                /* ` */ Tag::from(vec![2, 1]),
                /* y */ Tag::from(vec![2, 0]),
                /* z */ Tag::from(vec![2, 1, 0]),
            ]
        );
    }

    #[test]
    fn test_format_2() {
        let expr = expr::a(expr::a("W", "X"), expr::a("Y", "Z"));
        let formed = super::format(&expr);
        println!("{:?}", formed);
        assert_eq!(formed.expr, "``W X`Y Z");
        assert_eq!(
            formed.mapping,
            vec![
                /* ` */ Tag::from(vec![2]),
                /* ` */ Tag::from(vec![1]),
                /* W */ Tag::from(vec![0]),
                /*   */ Tag::from(vec![]),
                /* X */ Tag::from(vec![1, 0]),
                /* ` */ Tag::from(vec![2, 1]),
                /* Y */ Tag::from(vec![2, 0]),
                /*   */ Tag::from(vec![]),
                /* Z */ Tag::from(vec![2, 1, 0]),
            ]
        );
    }

    #[test]
    fn test_format_3() {
        let expr = expr::a(expr::a("FOO", "BAR"), expr::a("HOGE", "FUGA"));
        let formed = super::format(&expr);
        println!("{:?}", formed);
        assert_eq!(formed.expr, "``FOO BAR`HOGE FUGA");
        assert_eq!(
            formed.mapping,
            vec![
                /* ` */ Tag::from(vec![2]),
                /* ` */ Tag::from(vec![1]),
                /* F */ Tag::from(vec![0]),
                /* O */ Tag::from(vec![0]),
                /* O */ Tag::from(vec![0]),
                /*   */ Tag::from(vec![]),
                /* B */ Tag::from(vec![1, 0]),
                /* A */ Tag::from(vec![1, 0]),
                /* R */ Tag::from(vec![1, 0]),
                /* ` */ Tag::from(vec![2, 1]),
                /* H */ Tag::from(vec![2, 0]),
                /* O */ Tag::from(vec![2, 0]),
                /* G */ Tag::from(vec![2, 0]),
                /* E */ Tag::from(vec![2, 0]),
                /*   */ Tag::from(vec![]),
                /* F */ Tag::from(vec![2, 1, 0]),
                /* U */ Tag::from(vec![2, 1, 0]),
                /* G */ Tag::from(vec![2, 1, 0]),
                /* A */ Tag::from(vec![2, 1, 0]),
            ]
        );
    }

    #[test]
    fn test_format_4() {
        let expr = expr::a(expr::a("W", "x"), expr::a("y", "Z"));
        let formed = super::format(&expr);
        println!("{:?}", formed);
        assert_eq!(formed.expr, "``Wx`yZ");
        assert_eq!(
            formed.mapping,
            vec![
                Tag::from(vec![2]),
                Tag::from(vec![1]),
                Tag::from(vec![0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![2, 1]),
                Tag::from(vec![2, 0]),
                Tag::from(vec![2, 1, 0]),
            ]
        );
    }

    #[test]
    fn test_format_5() {
        let expr = expr::a(
            expr::a("w", expr::l("x", expr::a("x", "x"))),
            expr::a("y", "z"),
        );
        let formed = super::format(&expr);
        println!("{}", formed.expr);
        println!("");
        println!("{:?}", formed);
        println!("");
        assert_eq!(formed.expr, "``wλx.`xx`yz");
        assert_eq!(
            formed.mapping,
            vec![
                Tag::from(vec![2]),
                Tag::from(vec![1]),
                Tag::from(vec![0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![1, 0]),
                Tag::from(vec![2, 1]),
                Tag::from(vec![2, 0]),
                Tag::from(vec![2, 1, 0]),
            ]
        );
    }
}
