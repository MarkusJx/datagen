#[cfg(feature = "serialize")]
use serde::{Serialize, Serializer};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SchemaPath {
    pub path: VecDeque<String>,
}

impl SchemaPath {
    #[cfg(feature = "generate")]
    pub fn root() -> Self {
        Self {
            path: VecDeque::new(),
        }
    }

    #[cfg(feature = "map-schema")]
    pub fn append<S: ToString>(&self, path: S) -> SchemaPath {
        let mut res = self.path.clone();
        res.push_back(path.to_string());

        Self { path: res }
    }

    #[cfg(feature = "map-schema")]
    pub fn len(&self) -> usize {
        self.path.len()
    }

    #[cfg(feature = "map-schema")]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
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

        let mut path = self.path.clone();
        for _ in 0..num {
            assert!(
                path.pop_front().is_some(),
                "Tried to remove more elements from path {} than exist",
                self
            );
        }

        Self { path }
    }

    #[cfg(feature = "map-schema")]
    pub fn to_normalized_path(&self) -> String {
        self.path
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
            self.path.iter().cloned().collect::<Vec<_>>().join(".")
        )
    }
}
