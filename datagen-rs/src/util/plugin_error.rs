#[cfg(feature = "native-plugin")]
use crate::plugins::imported_plugin::PluginData;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
#[cfg(feature = "native-plugin")]
use std::rc::Rc;

pub(crate) struct PluginError {
    inner: Box<dyn Error>,
    #[cfg(feature = "native-plugin")]
    _plugin: Rc<PluginData>,
}

#[cfg(feature = "native-plugin")]
mod native {
    use crate::plugins::imported_plugin::ImportedPlugin;
    use crate::util::plugin_error::PluginError;
    use crate::util::types::Result;
    use std::error::Error;

    impl PluginError {
        fn from_error(
            error: Box<dyn Error>,
            plugin: &ImportedPlugin,
            func_name: &'static str,
        ) -> Self {
            Self {
                inner: format!(
                    "Failed to call function '{func_name}' on plugin '{}': {error}",
                    plugin.name(),
                )
                .into(),
                _plugin: plugin.get_data(),
            }
        }
    }

    pub(crate) trait MapPluginError<T> {
        /// Ensure a plugin throwing an error is still loaded once [`Result<T>::unwrap`]
        /// or similar is called on the [`Result<T>`]. If this is not called on a thrown
        /// error, the plugin may be un-loaded before the error value is retrieved,
        /// causing the program to crash.
        fn map_plugin_error(self, plugin: &ImportedPlugin, func_name: &'static str) -> Result<T>;
    }

    impl<T> MapPluginError<T> for Result<T> {
        fn map_plugin_error(self, plugin: &ImportedPlugin, func_name: &'static str) -> Result<T> {
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
