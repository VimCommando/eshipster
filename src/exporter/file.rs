use super::Export;
use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
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

    async fn write(&self, docs: Vec<ShardDoc>) -> Result<usize> {
        log::debug!("Writing docs to file {}", &self.path.display());
        let mut writer = BufWriter::new(&self.file);
        let mut doc_count = 0;
        for doc in docs {
            serde_json::to_writer(&mut writer, &doc)?;
            writeln!(&mut writer)?;
            doc_count += 1;
        }
        writer.flush()?;
        Ok(doc_count)
    }
}

impl std::fmt::Display for FileExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
