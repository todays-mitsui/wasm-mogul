#[derive(Clone, Debug, PartialEq)]
pub enum Path {
    Arg(Index, Box<Path>),
    Callee(Arity),
}

impl Path {
    pub fn set_arity(&mut self, arity: Arity) {
        match self {
            Path::Arg(_, next) => next.set_arity(arity),
            Path::Callee(old_arity) => *old_arity = arity,
        }
    }
}

type Index = usize;
type Arity = usize;

#[cfg(test)]
impl From<Path> for Vec<usize> {
    fn from(path: Path) -> Self {
        let mut indices = Vec::new();
        let mut path = path;
        loop {
            match path {
                Path::Arg(index, next) => {
                    indices.push(index);
                    path = *next;
                }
                Path::Callee(arity) => {
                    indices.push(arity);
                    break;
                }
            }
        }
        indices
    }
}

#[cfg(test)]
impl From<&Path> for Vec<usize> {
    fn from(path: &Path) -> Self {
        let mut indices = Vec::new();
        let mut path = path;
        loop {
            match path {
                Path::Arg(index, next) => {
                    indices.push(*index);
                    path = next;
                }
                Path::Callee(arity) => {
                    indices.push(*arity);
                    break;
                }
            }
        }
        indices
    }
}

// ========================================================================== //

pub struct PathBuilder {
    routes: Vec<Index>,
    arity: Option<Arity>,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            arity: None,
        }
    }

    pub fn add_route(&mut self, route: Index) {
        self.routes.push(route);
    }

    pub fn set_arity(&mut self, arity: Arity) {
        self.arity = Some(arity);
    }

    pub fn build(self) -> Path {
        let mut path = Path::Callee(self.arity.unwrap());
        for index in self.routes.into_iter().rev() {
            path = Path::Arg(index, Box::new(path));
        }
        path
    }
}
