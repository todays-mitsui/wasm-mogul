use crate::expr::Expr;
use crate::expr::Identifier;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Aliases(HashMap<Identifier, Expr>);

impl Aliases {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, id: &Identifier) -> Option<&Expr> {
        self.0.get(id)
    }

    pub fn has(&self, id: &Identifier) -> bool {
        self.0.contains_key(id)
    }
}

impl From<HashMap<Identifier, Expr>> for Aliases {
    fn from(map: HashMap<Identifier, Expr>) -> Self {
        Self(map)
    }
}
