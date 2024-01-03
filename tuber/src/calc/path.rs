#[derive(Clone, Debug, PartialEq)]
pub struct Path(Vec<Index>, Arity);

type Index = usize;
type Arity = usize;

impl Path {
    pub fn new(routes: Vec<Index>, arity: Arity) -> Self {
        Self(routes, arity)
    }
}

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
        Path::new(self.routes, self.arity.unwrap())
    }
}
