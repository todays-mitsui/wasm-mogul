use super::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// 変数
    Variable(Identifier),

    /// シンボル
    Symbol(Identifier),

    /// 適用
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },

    /// ラムダ抽象
    Lambda { param: Identifier, body: Box<Expr> },
}

impl From<&str> for Expr {
    fn from(s: &str) -> Self {
        match s.chars().nth(0) {
            Some(':') => Expr::Symbol((&s[1..]).into()),
            Some(_) => Expr::Variable(s.into()),
            _ => panic!("invalid identifier"),
        }
    }
}

impl From<String> for Expr {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

pub fn v<Id: Into<Identifier>>(id: Id) -> Expr {
    Expr::Variable(id.into())
}

pub fn s<Id: Into<Identifier>>(id: Id) -> Expr {
    Expr::Symbol(id.into())
}

pub fn a<L: Into<Expr>, R: Into<Expr>>(lhs: L, rhs: R) -> Expr {
    Expr::Apply {
        lhs: Box::new(lhs.into()),
        rhs: Box::new(rhs.into()),
    }
}

pub fn l<P: Into<Identifier>, B: Into<Expr>>(param: P, body: B) -> Expr {
    Expr::Lambda {
        param: param.into(),
        body: Box::new(body.into()),
    }
}
