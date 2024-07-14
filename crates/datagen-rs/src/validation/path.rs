use std::fmt::Display;

#[derive(Clone)]
pub struct ValidationPath {
    path: Vec<String>,
}

impl ValidationPath {
    pub fn root() -> Self {
        Self { path: vec![] }
    }

    pub fn append<S1: ToString, S2: ToString>(&self, first: S1, second: S2) -> Self {
        let mut path = self.path.clone();
        path.push(first.to_string());
        path.push(second.to_string());

        Self { path }
    }

    pub fn append_single<S: ToString>(&self, part: S) -> Self {
        let mut path = self.path.clone();
        path.push(part.to_string());

        Self { path }
    }
}

impl Display for ValidationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("."))
    }
}
