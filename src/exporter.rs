mod file;
mod stream;

use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use file::FileExporter;
use std::path::Path;
use stream::StreamExporter;

trait Export {
    fn write(&self, docs: Vec<ShardDoc>) -> Result<()>;
    fn is_connected(&self) -> bool;
}

pub enum Exporter {
    File(FileExporter),
    Stream(StreamExporter),
}

impl Exporter {
    pub fn parse(output: Option<&String>) -> Result<Self> {
        match output {
            None => Ok(Self::Stream(StreamExporter::new())),
            Some(output) => {
                let path = Path::new(output);
                let file_exporter = FileExporter::new(path.to_path_buf())?;
                Ok(Self::File(file_exporter))
            }
        }
    }

    pub fn write(&self, docs: Vec<ShardDoc>) -> Result<()> {
        match self {
            Self::File(exporter) => exporter.write(docs),
            Self::Stream(exporter) => exporter.write(docs),
        }
    }
}

impl std::fmt::Display for Exporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(exporter) => write!(f, "file {}", exporter),
            Self::Stream(exporter) => write!(f, "stream {}", exporter),
        }
    }
}
