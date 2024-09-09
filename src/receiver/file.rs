use super::Receive;
use crate::data::IndicesStats;
use color_eyre::eyre::Result;
use std::{fs::File, path::PathBuf};

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
    fn is_connected(&self) -> bool {
        let is_file = self.path.is_file();
        let filename = self.path.to_str().unwrap_or("");
        log::debug!("File {filename} is valid: {is_file}");
        is_file
    }

    fn read_indices_stats(&self) -> Result<IndicesStats> {
        log::debug!("Reading file: {}", self.path.display());
        let indices_stats: IndicesStats = serde_json::from_reader(&self.file)?;
        Ok(indices_stats)
    }
}

impl std::fmt::Display for FileReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
