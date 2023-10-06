use crate::backends::backend::{Backend, BackendConstructor};
use crate::objects::args::{BackendType, PluginArgs};
use crate::objects::geo_data::GeoFeature;
use anyhow::{anyhow, Context};
use rand::seq::SliceRandom;
use rand::thread_rng;
#[cfg(test)]
use std::any::Any;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

#[derive(Debug)]
pub(crate) struct MemoryBackend {
    files: Vec<AddressFile>,
}

#[derive(Debug)]
struct AddressFile {
    file: File,
    contents: Option<Vec<GeoFeature>>,
}

impl AddressFile {
    pub fn new(file: File) -> Self {
        Self {
            file,
            contents: None,
        }
    }

    fn get_contents(&mut self) -> anyhow::Result<&Vec<GeoFeature>> {
        if self.contents.is_none() {
            let reader = BufReader::new(&self.file);

            self.contents.replace(
                reader
                    .lines()
                    .map(|line| -> anyhow::Result<_> {
                        serde_json::from_str(&line?).map_err(Into::into)
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?,
            );
            self.file.rewind()?;
        }

        Ok(self.contents.as_ref().unwrap())
    }

    fn get_random_line(&mut self) -> anyhow::Result<GeoFeature> {
        self.get_contents()?
            .choose(&mut thread_rng())
            .cloned()
            .ok_or(anyhow!("Failed to get random address line"))
    }
}

impl Backend for MemoryBackend {
    fn get_random_feature(&mut self) -> anyhow::Result<GeoFeature> {
        self.files
            .choose_mut(&mut thread_rng())
            .ok_or(anyhow!("Failed to choose random address file"))?
            .get_random_line()
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[cfg(test)]
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl BackendConstructor for MemoryBackend {
    fn new(paths: Vec<String>, args: PluginArgs) -> anyhow::Result<Self> {
        if let BackendType::SQLite { .. } = args.backend.unwrap_or_default() {
            return Err(anyhow!(
                "Unable to create memory backend: The selected backend type is not memory"
            ));
        }

        Ok(Self {
            files: paths
                .into_iter()
                .map(|f| File::open(&f).context(anyhow!("Failed to open file: {}", f)))
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .map(AddressFile::new)
                .collect(),
        })
    }
}
