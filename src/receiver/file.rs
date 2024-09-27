use super::Receive;
use crate::data::{ElasticsearchApi, IndicesStats};
use color_eyre::eyre::Result;
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct FileReceiver {
    path: PathBuf,
    file: File,
}

impl FileReceiver {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(&path)?;
        Ok(Self { file, path })
    }
}

impl Receive for FileReceiver {
    async fn is_connected(&self) -> bool {
        let is_file = self.path.is_file();
        let filename = self.path.to_str().unwrap_or("");
        log::debug!("File {filename} is valid: {is_file}");
        is_file
    }

    async fn read_indices_stats(&self) -> Result<IndicesStats> {
        log::debug!("Reading file: {}", self.path.display());
        let reader = BufReader::new(&self.file);
        let indices_stats: IndicesStats = serde_json::from_reader(reader)?;
        Ok(indices_stats)
    }

    async fn get<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + ElasticsearchApi,
    {
        let path = PathBuf::from(T::file_name());
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let data: T = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

impl std::fmt::Display for FileReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
