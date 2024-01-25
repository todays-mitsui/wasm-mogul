use crate::expr::Path;
use std::ops::Range;

#[derive(Clone, PartialEq)]
pub struct Tag(Vec<usize>);

impl Tag {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&self, index: usize) -> Self {
        let mut indices = self.0.clone();
        indices.push(index);
        Self(indices)
    }

    pub fn replace_last(&self, index: usize) -> Self {
        let mut indices = self.0.clone();
        let last = self.0.len() - 1;
        indices[last] = index;
        Self(indices)
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "-");
        } else {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

impl From<Vec<usize>> for Tag {
    fn from(indices: Vec<usize>) -> Self {
        Self(indices)
    }
}

impl Path {
    pub fn range(&self, tags: &[Tag]) -> Option<Range<usize>> {
        let (prefix, arity) = self.prefix_and_arity();
        let len = prefix.len();

        let predicate = |tag: &Tag| {
            // println!("   tag: {:?}", tag.0);
            // println!("prefix: {:?}, {}", prefix, arity);
            prefix
                .iter()
                .enumerate()
                .all(|(i, v)| tag.0.get(i) == Some(v))
                && tag.0.get(len).map(|index| *index <= arity).unwrap_or(false)
        };

        let mut tags_iter = tags.iter();

        let start = tags_iter.position(predicate)?;
        let end = start
            + 1
            + tags_iter
                .position(|tag| !predicate(tag))
                .unwrap_or(tags.len() - 1);

        Some(start..end)
    }

    fn prefix_and_arity(&self) -> (Vec<usize>, usize) {
        let mut path = self;

        let mut prefix: Vec<usize> = Vec::new();
        while let Path::Arg(index, next) = path {
            prefix.push(*index);
            path = &**next;
        }

        let arity: usize = if let Path::Callee(arity) = path {
            *arity
        } else {
            panic!("Path::range() called on non-callee path");
        };

        (prefix, arity)
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let tags = vec![
            /*  0: a */ Tag::from(vec![0]),
            /*  1: ( */ Tag::from(vec![4]),
            /*  2: b */ Tag::from(vec![1, 0]),
            /*  3: , */ Tag::from(vec![4]),
            /*  4:   */ Tag::from(vec![4]),
            /*  5: c */ Tag::from(vec![2, 0]),
            /*  6: , */ Tag::from(vec![4]),
            /*  7:   */ Tag::from(vec![4]),
            /*  8: d */ Tag::from(vec![3, 0]),
            /*  9: ( */ Tag::from(vec![3, 2]),
            /* 10: e */ Tag::from(vec![3, 1, 0]),
            /* 11: , */ Tag::from(vec![3, 2]),
            /* 12:   */ Tag::from(vec![3, 2]),
            /* 13: f */ Tag::from(vec![3, 2, 0]),
            /* 14: ) */ Tag::from(vec![3, 2]),
            /* 15: , */ Tag::from(vec![4]),
            /* 16:   */ Tag::from(vec![4]),
            /* 17: g */ Tag::from(vec![4, 0]),
            /* 18: ( */ Tag::from(vec![4, 1]),
            /* 19: h */ Tag::from(vec![4, 1, 0]),
            /* 20: ) */ Tag::from(vec![4, 1]),
            /* 21: ) */ Tag::from(vec![4]),
        ];

        let path = Path::Callee(0);
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 0..1);

        let path = Path::Arg(1, Box::new(Path::Callee(0)));
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 2..3);

        let path = Path::Arg(3, Box::new(Path::Arg(1, Box::new(Path::Callee(0)))));
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 10..11);

        let path = Path::Arg(3, Box::new(Path::Callee(2)));
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 8..15);

        let path = Path::Arg(4, Box::new(Path::Callee(0)));
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 17..18);

        let path = Path::Arg(4, Box::new(Path::Callee(1)));
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 17..21);

        let path = Path::Callee(4);
        let range = path.range(&tags).unwrap();
        assert_eq!(range, 0..22);
    }
}
