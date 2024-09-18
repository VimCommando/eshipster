use super::Receive;
use crate::data::IndicesStats;
use color_eyre::eyre::Result;
use elasticsearch::{
    cert::CertificateValidation,
    http::{
        self,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
    indices::IndicesStatsParts,
    params::Level,
    Elasticsearch,
};
use url::Url;

pub struct ElasticsearchReceiver {
    client: Elasticsearch,
    url: Url,
}

impl ElasticsearchReceiver {
    pub fn new(url: Url) -> Self {
        let client =
            new_insecure_client(url.clone()).expect("Failed to create Elasticsearch client");

        Self { client, url }
    }
}

fn new_insecure_client(url: Url) -> Result<Elasticsearch> {
    let connection_pool = SingleNodeConnectionPool::new(url);
    let cert_validation = CertificateValidation::None;

    let transport = TransportBuilder::new(connection_pool)
        .header(http::headers::ACCEPT_ENCODING, "gzip".parse().unwrap())
        .cert_validation(cert_validation)
        .build()?;

    Ok(Elasticsearch::new(transport))
}

impl Receive for ElasticsearchReceiver {
    async fn is_connected(&self) -> bool {
        log::debug!("Testing Elasticsearch client connection");
        // An empty request to `/`
        let response = self
            .client
            .send(
                http::Method::Get,
                "",
                http::headers::HeaderMap::new(),
                Option::<&String>::None,
                Option::<&String>::None,
                None,
            )
            .await;

        match response {
            Ok(response) => {
                log::debug!(
                    "Elasticsearch client connection successful: {}",
                    response.status_code()
                );
                true
            }
            Err(e) => {
                log::error!("Elasticsearch client connection failed: {e}");
                false
            }
        }
    }

    async fn read_indices_stats(&self) -> Result<IndicesStats> {
        // '_all' is a wildcard to get stats for all indices
        let index_list = vec!["_all"];
        let indices_stats = self
            .client
            .indices()
            .stats(IndicesStatsParts::Index(&index_list))
            .level(Level::Shards)
            .send()
            .await?;

        // turbo-fish to use serde to parse the JSON response
        indices_stats
            .json::<IndicesStats>()
            .await
            .map_err(Into::into)
    }
}

impl std::fmt::Display for ElasticsearchReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
