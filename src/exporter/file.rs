use super::Export;
use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub struct FileExporter {
    file: File,
    path: PathBuf,
}

impl FileExporter {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)?;
        Ok(Self { file, path })
    }
}

impl Export for FileExporter {
    async fn is_connected(&self) -> bool {
        let is_file = self.path.is_file();
        let filename = self.path.to_str().unwrap_or("");
        log::debug!("File {filename} is valid: {is_file}");
        is_file
    }

    async fn write(&self, docs: Vec<ShardDoc>) -> Result<()> {
        log::info!(
            "Writing {} docs to file {}",
            docs.len(),
            &self.path.display()
        );
        for doc in docs {
            serde_json::to_writer(&self.file, &doc)?;
            writeln!(&self.file)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for FileExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
