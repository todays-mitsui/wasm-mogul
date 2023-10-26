use super::Identifier;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub struct BoundVars(HashSet<Identifier>);

impl BoundVars {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert<Id: Into<Identifier>>(&mut self, id: Id) {
        self.0.insert(id.into());
    }

    pub fn contains<Id: Into<Identifier>>(&self, id: Id) -> bool {
        self.0.contains(&id.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bound_vars() {
        let mut bound_vars = BoundVars::new();

        assert!(!bound_vars.contains("x"));
        assert!(!bound_vars.contains("y"));
        assert!(!bound_vars.contains("z"));

        bound_vars.insert("x");
        assert!(bound_vars.contains("x"));
        assert!(!bound_vars.contains("y"));
        assert!(!bound_vars.contains("z"));

        bound_vars.insert("y");
        assert!(bound_vars.contains("x"));
        assert!(bound_vars.contains("y"));
        assert!(!bound_vars.contains("z"));

        bound_vars.insert("z");
        assert!(bound_vars.contains("x"));
        assert!(bound_vars.contains("y"));
        assert!(bound_vars.contains("z"));
    }
}
