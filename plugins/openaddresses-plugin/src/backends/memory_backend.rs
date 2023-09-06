use crate::backends::backend::{Backend, BackendConstructor};
use crate::objects::args::{BackendType, PluginArgs};
use crate::objects::geo_data::GeoFeature;
use datagen_rs::util::types::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
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

    fn get_contents(&mut self) -> Result<&Vec<GeoFeature>> {
        if self.contents.is_none() {
            let reader = BufReader::new(&self.file);

            self.contents.replace(
                reader
                    .lines()
                    .map(|line| -> Result<_> { serde_json::from_str(&line?).map_err(Into::into) })
                    .collect::<Result<Vec<_>>>()?,
            );
            self.file.rewind()?;
        }

        Ok(self.contents.as_ref().unwrap())
    }

    fn get_random_line(&mut self) -> Result<GeoFeature> {
        self.get_contents()?
            .choose(&mut thread_rng())
            .cloned()
            .ok_or("Failed to get random address line".into())
    }
}

impl Backend for MemoryBackend {
    fn get_random_feature(&mut self) -> Result<GeoFeature> {
        self.files
            .choose_mut(&mut thread_rng())
            .ok_or("Failed to choose random address file".to_string())?
            .get_random_line()
    }
}

impl BackendConstructor for MemoryBackend {
    fn new(paths: Vec<String>, args: PluginArgs) -> Result<Self> {
        if let BackendType::SQLite { .. } = args.backend.unwrap_or_default() {
            return Err(
                "Unable to create memory backend: The selected backend type is not memory".into(),
            );
        }

        Ok(Self {
            files: paths
                .into_iter()
                .map(File::open)
                .map(|r| r.map_err(Into::into))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(AddressFile::new)
                .collect(),
        })
    }
}
