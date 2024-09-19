mod elasticsearch;
mod file;
mod stream;

use crate::client::{Auth, AuthType};
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
    pub async fn is_connected(&self) -> bool {
        match self {
            Self::Elasticsearch(exporter) => exporter.is_connected().await,
            Self::File(exporter) => exporter.is_connected().await,
            Self::Stream(exporter) => exporter.is_connected().await,
        }
    }

    pub fn parse(output: Option<&String>, auth_type: &AuthType) -> Result<Self> {
        log::debug!("Parsing exporter: {:?}", output);
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
                let auth = Auth::new(
                    auth_type,
                    std::env::var("ESHIPSTER_XP_USERNAME").ok(),
                    std::env::var("ESHIPSTER_XP_PASSWORD").ok(),
                    std::env::var("ESHIPSTER_XP_APIKEY").ok(),
                );
                let exporter = ElasticsearchExporter::new(url, auth)?;
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
