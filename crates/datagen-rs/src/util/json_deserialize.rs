use serde::de::{DeserializeOwned, Error};
use serde_json::Value;
use shellexpand::LookupError;
use std::collections::HashMap;

pub fn from_value<T>(mut value: Value) -> serde_json::Result<T>
where
    T: DeserializeOwned,
{
    map_value(&mut value, &std::env::vars().collect())?;
    serde_json::from_value(value)
}

pub fn from_reader<R, T>(rdr: R) -> serde_json::Result<T>
where
    R: std::io::Read,
    T: DeserializeOwned,
{
    let val = serde_json::from_reader(rdr)?;
    from_value(val)
}

pub fn from_str<T>(s: &str) -> serde_json::Result<T>
where
    T: DeserializeOwned,
{
    let val = serde_json::from_str(s)?;
    from_value(val)
}

fn map_value(value: &mut Value, env: &HashMap<String, String>) -> serde_json::Result<()> {
    match value {
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                map_value(v, env)?;
            }
        }
        Value::Array(vec) => {
            for v in vec.iter_mut() {
                map_value(v, env)?;
            }
        }
        Value::String(s) => {
            *value = Value::String(
                shellexpand::env_with_context(s, |name| Ok(env.get(name).cloned()))
                    .map_err(|e: LookupError<String>| serde_json::Error::custom(e.to_string()))?
                    .to_string(),
            );
        }
        _ => {}
    }

    Ok(())
}
