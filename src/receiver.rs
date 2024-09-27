mod elasticsearch;
mod file;

use crate::client::{Auth, AuthType, Host};
use crate::config;
use crate::data::{ElasticsearchApi, IndicesStats};
use color_eyre::eyre::{eyre, Result};
use elasticsearch::ElasticsearchReceiver;
use file::FileReceiver;
use serde::de::DeserializeOwned;
use std::path::Path;
use url::Url;

trait Receive {
    async fn is_connected(&self) -> bool;
    async fn read_indices_stats(&self) -> Result<IndicesStats>;
    async fn get<T>(&self) -> Result<T>
    where
        T: ElasticsearchApi + DeserializeOwned;
}

pub enum Receiver {
    File(FileReceiver),
    Elasticsearch(ElasticsearchReceiver),
}

impl Receiver {
    pub fn parse(input: &str, auth_type: &AuthType) -> Result<Self> {
        log::debug!("Parsing receiver: {}", input);
        match Host::parse(input) {
            Some(host) => {
                let receiver = ElasticsearchReceiver::from_host(host)?;
                return Ok(Self::Elasticsearch(receiver));
            }
            None => log::debug!("Input was not a known host"),
        }
        // Attempt to parse the input as a URL
        match Url::parse(input) {
            Ok(url) => {
                let auth = Auth::new(
                    auth_type,
                    config::ESHIPSTER_RC_USERNAME.clone(),
                    config::ESHIPSTER_RC_PASSWORD.clone(),
                    config::ESHIPSTER_RC_APIKEY.clone(),
                );
                let receiver = ElasticsearchReceiver::new(url, auth)?;
                return Ok(Self::Elasticsearch(receiver));
            }
            Err(_) => log::debug!("Input was not a valid URL"),
        };

        // Fallback to a file path
        let path = Path::new(&input);
        match path.is_file() {
            true => {
                let file_receiver = FileReceiver::new(path.to_path_buf())?;
                return Ok(Self::File(file_receiver));
            }
            false => Err(eyre!("Could not parse input")),
        }
    }

    pub async fn read_indices_stats(&self) -> Result<IndicesStats> {
        match self {
            Receiver::File(file_receiver) => file_receiver.read_indices_stats().await,
            Receiver::Elasticsearch(elasticsearch_receiver) => {
                elasticsearch_receiver.read_indices_stats().await
            }
        }
    }

    pub async fn get<T>(&self) -> Result<T>
    where
        T: ElasticsearchApi + DeserializeOwned,
    {
        match self {
            Receiver::File(file_receiver) => file_receiver.get::<T>().await,
            Receiver::Elasticsearch(elasticsearch_receiver) => {
                elasticsearch_receiver.get::<T>().await
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
