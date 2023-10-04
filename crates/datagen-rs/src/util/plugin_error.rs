#[cfg(feature = "native-plugin")]
use crate::plugins::imported_plugin::PluginData;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
#[cfg(feature = "native-plugin")]
use std::sync::Arc;

pub(crate) struct PluginError {
    inner: anyhow::Error,
    #[cfg(feature = "native-plugin")]
    _plugin: Arc<PluginData>,
}

unsafe impl Send for PluginError {}
unsafe impl Sync for PluginError {}

#[cfg(feature = "native-plugin")]
pub mod native {
    use crate::plugins::imported_plugin::ImportedPlugin;
    use crate::plugins::plugin::Plugin;
    use crate::util::plugin_error::PluginError;
    use anyhow::anyhow;

    impl PluginError {
        fn from_error(
            error: anyhow::Error,
            plugin: &ImportedPlugin,
            func_name: &'static str,
        ) -> Self {
            Self {
                inner: anyhow!(
                    "Failed to call function '{func_name}' on plugin '{}': {error}",
                    plugin.name(),
                )
                .into(),
                _plugin: plugin.get_data(),
            }
        }
    }

    pub(crate) trait MapPluginError<T> {
        /// Ensure a plugin throwing an error is still loaded once [`anyhow::Result<T>::unwrap`]
        /// or similar is called on the [`anyhow::Result<T>`]. If this is not called on a thrown
        /// error, the plugin may be un-loaded before the error value is retrieved,
        /// causing the program to crash.
        fn map_plugin_error(
            self,
            plugin: &ImportedPlugin,
            func_name: &'static str,
        ) -> anyhow::Result<T>;
    }

    impl<T> MapPluginError<T> for anyhow::Result<T> {
        fn map_plugin_error(
            self,
            plugin: &ImportedPlugin,
            func_name: &'static str,
        ) -> anyhow::Result<T> {
            self.map_err(|e| PluginError::from_error(e, plugin, func_name).into())
        }
    }
}

impl Debug for PluginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for PluginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error for PluginError {}
