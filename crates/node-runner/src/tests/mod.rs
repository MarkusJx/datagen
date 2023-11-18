#[cfg(feature = "nodejs")]
mod node_plugin;

#[macro_export]
macro_rules! json_string {
    ($($json:tt)+) => {
        serde_json::to_string(&serde_json::json!($($json)+)).unwrap()
    };
}

fn error_to_string(e: anyhow::Error) -> String {
    e.chain()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\nCaused by: ")
}

fn assert_error_eq(e1: anyhow::Error, e2: anyhow::Error) {
    assert_eq!(error_to_string(e1), error_to_string(e2));
}
