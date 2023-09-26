#[napi]
pub struct SchemaPath(datagen_rs::generate::schema_path::SchemaPath);

#[napi]
impl SchemaPath {
    pub fn from_path(inner: datagen_rs::generate::schema_path::SchemaPath) -> SchemaPath {
        Self(inner)
    }

    #[napi]
    pub fn append(&self, path: String) -> SchemaPath {
        Self::from_path(self.0.append(path))
    }

    #[napi]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[napi]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[napi]
    pub fn to_normalized_path(&self) -> String {
        self.0.to_normalized_path()
    }

    #[napi]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
