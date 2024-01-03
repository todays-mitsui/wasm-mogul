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

// ========================================================================== //

// #[derive(Clone, Debug, PartialEq)]
// pub struct Path(Vec<Index>, Arity);

// type Index = usize;
// type Arity = usize;

// impl Path {
//     pub fn new(routes: Vec<Index>, arity: Arity) -> Self {
//         Self(routes, arity)
//     }

//     pub fn set_arity(&mut self, arity: Arity) {
//         self.1 = arity;
//     }

//     pub fn shift(&mut self) -> Option<Index> {
//         if self.0.is_empty() {
//             None
//         } else {
//             Some(self.0.remove(0))
//         }
//     }

//     pub fn arity(&self) -> Arity {
//         self.1
//     }
// }

// pub struct PathBuilder {
//     routes: Vec<Index>,
//     arity: Option<Arity>,
// }

// impl PathBuilder {
//     pub fn new() -> Self {
//         Self {
//             routes: Vec::new(),
//             arity: None,
//         }
//     }

//     pub fn add_route(&mut self, route: Index) {
//         self.routes.push(route);
//     }

//     pub fn set_arity(&mut self, arity: Arity) {
//         self.arity = Some(arity);
//     }

//     pub fn build(self) -> Path {
//         Path::new(self.routes, self.arity.unwrap())
//     }
// }
