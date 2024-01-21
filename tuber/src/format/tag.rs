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
