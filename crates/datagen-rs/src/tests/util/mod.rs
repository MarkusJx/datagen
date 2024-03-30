use crate::generate::current_schema::{CurrentSchema, CurrentSchemaRef};
use crate::plugins::plugin_list::PluginList;
use crate::schema::schema_definition::SchemaOptions;

#[cfg(feature = "env-schema")]
mod json_deserialize;

pub(in crate::tests) fn root_schema() -> CurrentSchemaRef {
    CurrentSchema::root(
        SchemaOptions {
            plugins: None,
            serializer: None,
            max_ref_cache_size: None,
            ignore_not_found_local_refs: None,
            serialize_non_strings: None,
        }
        .into(),
        PluginList::empty().into(),
    )
}

#[macro_export]
macro_rules! assert_enum {
    ($enum:expr, $variant: path) => {
        match $enum {
            $variant(val) => val,
            _ => panic!("Expected {}, got {:?}", stringify!($variant), $enum),
        }
    };
    () => {};
}

#[macro_export]
macro_rules! generate_schema {
        ($($json:tt)+) => {
            serde_json::from_value::<$crate::schema::any_value::AnyValue>(
                serde_json::json!($($json)+)
            )
                .unwrap()
                .into_random($crate::tests::util::root_schema())
        };
    }
