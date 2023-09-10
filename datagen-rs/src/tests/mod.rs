mod plugins;

mod util {
    use crate::generate::current_schema::{CurrentSchema, CurrentSchemaRef};
    use crate::plugins::plugin_list::PluginList;
    use crate::schema::schema_definition::SchemaOptions;

    pub(in crate::tests) fn root_schema() -> CurrentSchemaRef {
        CurrentSchema::root(
            SchemaOptions {
                plugins: None,
                serializer: None,
                max_ref_cache_size: None,
                ignore_not_found_local_refs: None,
                serialize_refs: None,
            }
            .into(),
            PluginList::empty().into(),
        )
    }
}
