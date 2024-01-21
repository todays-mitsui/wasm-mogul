#[derive(Clone, Debug, PartialEq)]
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
