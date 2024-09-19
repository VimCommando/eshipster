use super::Export;
use crate::client::{Auth, ElasticsearchBuilder};
use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use elasticsearch::{BulkOperation, BulkParts, Elasticsearch};
use url::Url;

pub struct ElasticsearchExporter {
    client: Elasticsearch,
    url: Url,
}

impl ElasticsearchExporter {
    pub fn new(url: Url, auth: Auth) -> Result<Self> {
        let client = ElasticsearchBuilder::new(url.clone())
            .insecure(true)
            .auth(auth)
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
        let status_code = match self
            .client
            .send(
                elasticsearch::http::Method::Get,
                "",
                elasticsearch::http::headers::HeaderMap::new(),
                Option::<&String>::None,
                Option::<&String>::None,
                None,
            )
            .await
        {
            Ok(res) => {
                log::debug!("{:?}", res);
                res.status_code().as_str().to_string()
            }
            Err(e) => {
                log::error!("{e}");
                "599".to_string()
            }
        };

        status_code == "200"
    }
}

impl std::fmt::Display for ElasticsearchExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
