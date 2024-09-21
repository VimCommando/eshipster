mod elasticsearch;
mod file;
mod stream;

use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use elasticsearch::ElasticsearchExporter;
use file::FileExporter;
use std::path::Path;
use stream::StreamExporter;
use url::Url;

trait Export {
    async fn write(&self, docs: Vec<ShardDoc>) -> Result<()>;
    async fn is_connected(&self) -> bool;
}

pub enum Exporter {
    Elasticsearch(ElasticsearchExporter),
    File(FileExporter),
    Stream(StreamExporter),
}

impl Exporter {
    pub fn parse(output: Option<&String>) -> Result<Self> {
        // No output given, write to stdout
        let output = match output {
            None => {
                let exporter = StreamExporter::new();
                return Ok(Self::Stream(exporter));
            }
            Some(output) => output,
        };
        // Attempt to parse the output as a URL
        match Url::parse(output) {
            Ok(url) => {
                let exporter = ElasticsearchExporter::new(url)?;
                return Ok(Self::Elasticsearch(exporter));
            }
            Err(_) => log::debug!("Output was not a valid URL"),
        };
        // Fallback to a file path
        let path = Path::new(output);
        let exporter = FileExporter::new(path.to_path_buf())?;
        Ok(Self::File(exporter))
    }

    pub async fn write(&self, docs: Vec<ShardDoc>) -> Result<()> {
        match self {
            Self::Elasticsearch(exporter) => exporter.write(docs).await,
            Self::File(exporter) => exporter.write(docs).await,
            Self::Stream(exporter) => exporter.write(docs).await,
        }
    }
}

impl std::fmt::Display for Exporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elasticsearch(exporter) => write!(f, "elasticsearch {}", exporter),
            Self::File(exporter) => write!(f, "file {}", exporter),
            Self::Stream(exporter) => write!(f, "stream {}", exporter),
        }
    }
}
