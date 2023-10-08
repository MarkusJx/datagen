#[cfg(feature = "nodejs")]
mod node_plugin;

#[macro_export]
macro_rules! json_string {
    ($($json:tt)+) => {
        serde_json::to_string(&serde_json::json!($($json)+)).unwrap()
    };
}
