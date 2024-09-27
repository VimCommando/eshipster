use super::Receive;
use crate::data::{ElasticsearchApi, IndicesStats};
use color_eyre::eyre::{eyre, Result};
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct DirectoryReceiver {
    path: PathBuf,
}

impl DirectoryReceiver {
    pub fn new(path: PathBuf) -> Result<Self> {
        match path.is_dir() {
            true => {
                log::debug!("Directory is valid: {}", path.display());
                Ok(Self { path })
            }
            false => {
                log::debug!("Directory is invalid: {}", path.display());
                Err(eyre!(
                    "Filesystme input must be a directory: {}",
                    path.display()
                ))
            }
        }
    }
}

impl Receive for DirectoryReceiver {
    async fn is_connected(&self) -> bool {
        let is_file = self.path.is_file();
        let filename = self.path.to_str().unwrap_or("");
        log::debug!("File {filename} is valid: {is_file}");
        is_file
    }

    async fn get<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + ElasticsearchApi,
    {
        let filename = &self.path.join(T::file_name());
        log::debug!("Reading file: {}", &filename.display());
        let file = File::open(&filename)?;
        let reader = BufReader::new(file);
        let data: T = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

impl std::fmt::Display for DirectoryReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
