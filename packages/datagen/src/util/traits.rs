use datagen_rs::util::types::Result;

pub trait IntoNapiResult<T> {
    fn into_napi(self) -> napi::Result<T>;
}

impl<T> IntoNapiResult<T> for Result<T> {
    fn into_napi(self) -> napi::Result<T> {
        self.map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}
