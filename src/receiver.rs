mod elasticsearch;
mod file;

use crate::data::IndicesStats;
use color_eyre::eyre::{eyre, Result};
use elasticsearch::ElasticsearchReceiver;
use file::FileReceiver;
use std::path::Path;
use url::Url;

trait Receive {
    async fn is_connected(&self) -> bool;
    async fn read_indices_stats(&self) -> Result<IndicesStats>;
}

pub enum Receiver {
    File(FileReceiver),
    Elasticsearch(ElasticsearchReceiver),
}

impl Receiver {
    pub fn parse(input: &str) -> Result<Self> {
        match Url::parse(input) {
            Ok(url) => {
                let es_receiver = ElasticsearchReceiver::new(url);
                return Ok(Self::Elasticsearch(es_receiver));
            }
            Err(_) => log::debug!("Input did not parse as a URL"),
        };

        let path = Path::new(&input);
        if path.is_file() {
            let file_receiver = FileReceiver::new(path.to_path_buf())?;
            return Ok(Self::File(file_receiver));
        }

        Err(eyre!("Could not parse input"))
    }

    pub async fn read_indices_stats(&self) -> Result<IndicesStats> {
        match self {
            Receiver::File(file_receiver) => file_receiver.read_indices_stats().await,
            Receiver::Elasticsearch(elasticsearch_receiver) => {
                elasticsearch_receiver.read_indices_stats().await
            }
        }
    }
}

impl std::fmt::Display for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Receiver::File(file_receiver) => write!(f, "file {}", file_receiver),
            Receiver::Elasticsearch(elasticsearch_receiver) => {
                write!(f, "elasticsearch {}", elasticsearch_receiver)
            }
        }
    }
}
