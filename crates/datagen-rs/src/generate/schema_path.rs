#[cfg(feature = "serialize")]
use serde::{Serialize, Serializer};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SchemaPath(pub VecDeque<String>);

impl SchemaPath {
    #[cfg(feature = "generate")]
    pub fn root() -> Self {
        Self(VecDeque::new())
    }

    #[cfg(feature = "map-schema")]
    pub fn append<S: ToString>(&self, path: S) -> SchemaPath {
        let mut res = self.0.clone();
        res.push_back(path.to_string());
        Self(res)
    }

    #[cfg(feature = "map-schema")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[cfg(feature = "map-schema")]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[cfg(any())]
    pub fn normalized_len(&self) -> usize {
        self.0
            .iter()
            .filter(|s| !s.chars().all(|c| c.is_numeric()))
            .count()
    }

    #[cfg(feature = "map-schema")]
    pub fn pop(&self, num: i32) -> SchemaPath {
        if num < 0 {
            return self.clone();
        }

        let mut res = self.0.clone();
        for _ in 0..num {
            assert!(
                res.pop_front().is_some(),
                "Tried to remove more elements from path {} than exist",
                self
            );
        }

        Self(res)
    }

    #[cfg(feature = "map-schema")]
    pub fn to_normalized_path(&self) -> String {
        self.0
            .iter()
            .filter(|s| !s.chars().all(|c| c.is_numeric()))
            .cloned()
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[cfg(feature = "serialize")]
impl Serialize for SchemaPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for SchemaPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().cloned().collect::<Vec<_>>().join(".")
        )
    }
}
