use crate::expr::Identifier;
use crate::func::Func;
use regex::Regex;
use std::collections::HashMap;

/// 定義済みの名前空間を表現する
///
/// 識別子と関数の組を保持する
#[derive(Debug, Clone, PartialEq)]
pub struct Context(HashMap<Identifier, Func>);

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, id: &Identifier) -> Option<&Func> {
        self.0.get(id)
    }

    pub fn def(&mut self, func: Func) {
        self.0.insert(func.name().into(), func);
    }

    pub fn del(&mut self, id: &Identifier) {
        self.0.remove(id);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &Func)> {
        self.0.iter()
    }

    #[cfg(test)]
    pub fn arity(&self, id: &str) -> Option<usize> {
        self.0.get(&id.into()).map(|f| f.arity())
    }

    #[cfg(test)]
    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn to_vec(self) -> Vec<Func> {
        let mut vec = self
            .0
            .into_iter()
            .map(|(_, f)| feature(f))
            .collect::<Vec<(Feature, Func)>>();

        vec.sort_by(|l, r| {
            let l = &l.0;
            let r = &r.0;
            r.short
                .cmp(&l.short)
                .then(l.name.to_lowercase().cmp(&r.name.to_lowercase()))
                .then(r.name.cmp(&l.name))
                .then(l.index.cmp(&r.index))
        });

        vec.into_iter().map(|(_, f)| f).collect()
    }
}

impl From<Vec<Func>> for Context {
    fn from(v: Vec<Func>) -> Self {
        let mut context = HashMap::new();
        for func in v {
            context.insert(func.name().into(), func);
        }
        Self(context)
    }
}

// ========================================================================== //

struct Feature {
    short: bool,
    name: String,
    index: Option<usize>,
}

fn feature<'a>(func: Func) -> (Feature, Func) {
    let pattern = Regex::new(r"\A(.*?)(\d*)\z").unwrap();

    let name = func.name();
    let (name, index) = match pattern.captures(name).map(|c| c.extract()) {
        Some((_, [s, n])) => (s, usize::from_str_radix(n, 10).ok()),
        None => (name, None),
    };

    let short = index.is_none() && name.len() == 1;

    (
        Feature {
            short,
            name: name.to_string(),
            index,
        },
        func,
    )
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
