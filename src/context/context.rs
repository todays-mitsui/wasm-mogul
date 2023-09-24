use crate::expr::Identifier;
use crate::func::Func;
use std::collections::HashMap;

/// 定義済みの名前空間を表現する
///
/// 識別子と関数の組を保持する
#[derive(Debug, Clone, PartialEq)]
pub struct Context(HashMap<Identifier, Func>);

impl Context {
    pub fn arity(&self, id: &str) -> Option<usize> {
        self.0.get(&id.into()).map(|f| f.arity())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &Func)> {
        self.0.iter()
    }

    #[cfg(test)]
    pub fn count(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Func>> for Context {
    fn from(v: Vec<Func>) -> Self {
        let mut context = HashMap::new();
        for func in v {
            context.insert(func.name().into(), func);
        }
        Context(context)
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::func;

    fn setup() -> Context {
        let i = func::new("i", vec!["x"], "x");
        let k = func::new("k", vec!["x", "y"], "x");
        let s = func::new(
            "s",
            vec!["x", "y", "z"],
            expr::a(expr::a("x", "z"), expr::a("y", "z")),
        );

        Context::from(vec![i, k, s])
    }

    #[test]
    fn test_context_from() {
        let context: Context = setup();

        assert_eq!(context.count(), 3);
        assert_eq!(context.arity("i"), Some(1));
        assert_eq!(context.arity("k"), Some(2));
        assert_eq!(context.arity("s"), Some(3));
        assert_eq!(context.arity("UNDEFINED"), None);
    }

    #[test]
    fn test_arity() {
        let context: Context = setup();

        assert_eq!(context.arity("i"), Some(1));
        assert_eq!(context.arity("k"), Some(2));
        assert_eq!(context.arity("s"), Some(3));
        assert_eq!(context.arity("UNDEFINED"), None);
    }

    #[test]
    fn test_iter() {
        let context: Context = setup();

        for (id, f) in context.iter() {
            match id.as_str() {
                "i" => assert_eq!(f.arity(), 1),
                "k" => assert_eq!(f.arity(), 2),
                "s" => assert_eq!(f.arity(), 3),
                _ => panic!("unexpected function: {}", id),
            }
        }
    }
}
