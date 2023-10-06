pub trait IntoNapiResult<T> {
    fn into_napi(self) -> napi::Result<T>;
}

impl<T> IntoNapiResult<T> for anyhow::Result<T> {
    fn into_napi(self) -> napi::Result<T> {
        self.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }
}
