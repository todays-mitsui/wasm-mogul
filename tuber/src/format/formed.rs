use super::tag::Tag;

pub struct Formed {
    pub expr: String,
    pub mapping: Vec<Tag>,
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
