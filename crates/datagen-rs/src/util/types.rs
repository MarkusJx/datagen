use std::error::Error;

pub type AnyError = Box<dyn Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, AnyError>;
