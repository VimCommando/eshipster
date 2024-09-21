use super::Export;
use crate::client::ElasticsearchBuilder;
use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use elasticsearch::{BulkOperation, BulkParts, Elasticsearch};
use url::Url;

pub struct ElasticsearchExporter {
    client: Elasticsearch,
    url: Url,
}

impl ElasticsearchExporter {
    pub fn new(url: Url) -> Result<Self> {
        let client = ElasticsearchBuilder::new(url.clone())
            .insecure(true)
            .build()?;

        Ok(Self { client, url })
    }
}

impl Export for ElasticsearchExporter {
    async fn write(&self, docs: Vec<ShardDoc>) -> Result<()> {
        let index = "eshipster-shards";
        let ops: Vec<BulkOperation<serde_json::Value>> = docs
            .into_iter()
            .map(|doc| BulkOperation::create(doc.as_value()).into())
            .collect();

        self.client
            .bulk(BulkParts::Index(&index))
            .body(ops)
            .send()
            .await?;

        Ok(())
    }

    async fn is_connected(&self) -> bool {
        true
    }
}

impl std::fmt::Display for ElasticsearchExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
