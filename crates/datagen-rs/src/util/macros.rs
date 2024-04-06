#[macro_export]
macro_rules! bail_unsupported {
    ($feature: expr) => {
        anyhow::bail!(
            "{} is not supported without the '{}' feature",
            $crate::function!(),
            $feature
        )
    };
}

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}
