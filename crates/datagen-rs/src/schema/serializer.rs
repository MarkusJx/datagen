#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The serializer to use when serializing the generated data.
/// If not specified, the default is JSON.
/// The serializer is specified in the schema options.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase", tag = "type"))]
pub enum Serializer {
    /// The JSON serializer.
    Json {
        /// Whether to pretty print the JSON.
        /// If not specified, the default is false.
        pretty: Option<bool>,
    },
    /// The YAML serializer.
    Yaml,
    /// The XML serializer.
    /// The root element must be specified.
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Xml {
        /// The name of the root element.
        root_element: String,
        /// Whether to pretty print the XML.
        /// If not specified, the default is false.
        pretty: Option<bool>,
    },
    /// A plugin serializer.
    #[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
    Plugin {
        /// The name of the plugin.
        plugin_name: String,
        /// The arguments to pass to the plugin.
        args: Option<Value>,
    },
}

impl Default for Serializer {
    fn default() -> Self {
        Serializer::Json { pretty: None }
    }
}

impl Default for &Serializer {
    fn default() -> Self {
        &Serializer::Json { pretty: None }
    }
}

#[cfg(feature = "generate")]
pub mod generate {
    use crate::generate::generated_schema::GeneratedSchema;
    use crate::plugins::plugin_list::PluginList;
    use crate::schema::serializer::Serializer;
    use anyhow::anyhow;
    use std::io::Read;
    use std::sync::Arc;
    use xml::{EmitterConfig, ParserConfig};

    fn format_xml<R: Read>(src: R) -> anyhow::Result<String> {
        let mut dest = Vec::new();
        let reader = ParserConfig::new()
            .trim_whitespace(true)
            .ignore_comments(false)
            .create_reader(src);
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .normalize_empty_elements(false)
            .autopad_comments(false)
            .create_writer(&mut dest);

        for event in reader {
            if let Some(event) = event?.as_writer_event() {
                writer.write(event)?;
            }
        }

        String::from_utf8(dest).map_err(Into::into)
    }

    impl Serializer {
        pub fn serialize_generated(
            &self,
            generated: Arc<GeneratedSchema>,
            plugins: Option<Arc<PluginList>>,
        ) -> anyhow::Result<String> {
            match self {
                Serializer::Json { pretty } => pretty
                    .unwrap_or(false)
                    .then(|| serde_json::to_string_pretty(&generated))
                    .unwrap_or_else(|| serde_json::to_string(&generated))
                    .map_err(Into::into),
                Serializer::Yaml => serde_yaml::to_string(&generated).map_err(Into::into),
                Serializer::Xml {
                    root_element,
                    pretty,
                } => {
                    let res = quick_xml::se::to_string_with_root(root_element, &generated)
                        .map_err(anyhow::Error::new)?;

                    if pretty.unwrap_or(false) {
                        format_xml(res.as_bytes())
                    } else {
                        Ok(res)
                    }
                }
                Serializer::Plugin { plugin_name, args } => plugins
                    .ok_or(anyhow!("A plugin serializer is not allowed at this point"))?
                    .get(plugin_name)?
                    .serialize(&generated, args.clone().unwrap_or_default()),
            }
        }

        pub fn serialize_generated_with_progress(
            &self,
            generated: Arc<GeneratedSchema>,
            plugins: Option<Arc<PluginList>>,
            callback: &dyn Fn(usize, usize),
        ) -> anyhow::Result<String> {
            match self {
                Serializer::Plugin { plugin_name, args } => plugins
                    .ok_or(anyhow!("A plugin serializer is not allowed at this point"))?
                    .get(plugin_name)?
                    .serialize_with_progress(
                        &generated,
                        args.clone().unwrap_or_default(),
                        callback,
                    ),
                _ => self.serialize_generated(generated, plugins),
            }
        }
    }
}
