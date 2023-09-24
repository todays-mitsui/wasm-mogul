use crate::expr::Expr;
use crate::expr::Identifier;

/// 定義済み関数を表現する
///
/// 関数とラムダ抽象はよく似ているが、関数が 0 以上の arity を持つ点で異なる
#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    name: Identifier,
    params: Vec<Identifier>,
    body: Expr,
}

impl Func {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn params(&self) -> &Vec<Identifier> {
        &self.params
    }

    pub fn body(&self) -> &Expr {
        &self.body
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

pub fn new<Name, Param, Body>(name: Name, params: Vec<Param>, body: Body) -> Func
where
    Name: Into<Identifier>,
    Param: Into<Identifier>,
    Body: Into<Expr>,
{
    Func {
        name: name.into(),
        params: params.into_iter().map(|i| i.into()).collect(),
        body: body.into(),
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    #[test]
    fn test_name() {
        let f = new("i", vec!["x"], "x");
        assert_eq!(f.name(), "i");
    }

    #[test]
    fn test_params() {
        let f = new("k", vec!["x", "y"], "x");
        assert_eq!(f.params(), &vec!["x".into(), "y".into()]);
    }

    #[test]
    fn test_body() {
        let f = new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        );
        assert_eq!(f.body(), &expr::a(expr::a("x", "z"), expr::a("y", "z")));
    }

    #[test]
    fn test_arity() {
        let f = new("i", vec!["x"], "x");
        assert_eq!(f.arity(), 1);

        let f = new("k", vec!["x", "y"], "x");
        assert_eq!(f.arity(), 2);

        let f = new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        );
        assert_eq!(f.arity(), 3);
    }
}
