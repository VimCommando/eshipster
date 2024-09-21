use super::Export;
use crate::client::{Auth, ElasticsearchBuilder, Host};
use crate::data::ShardDoc;
use color_eyre::eyre::Result;
use elasticsearch::{
    http::{headers, request::JsonBody, response::Response, Method},
    BulkOperation, BulkParts, Elasticsearch,
};
use serde_json::Value;
use url::Url;

pub struct ElasticsearchExporter {
    client: Elasticsearch,
    url: Url,
}

impl ElasticsearchExporter {
    /// Create a new ElasticsearchExporter from a URL and Auth
    pub fn new(url: Url, auth: Auth) -> Result<Self> {
        let client = ElasticsearchBuilder::new(url.clone())
            .insecure(true)
            .auth(auth)
            .build()?;

        Ok(Self { client, url })
    }

    /// Create a new ElasticsearchExporter from a Host
    pub fn from_host(host: Host) -> Result<Self> {
        let url = host.get_url();
        let client = ElasticsearchBuilder::from_host(host)?;
        Ok(Self { client, url })
    }

    /// Send a request to an arbitrary path on the Elasticsearch client
    pub async fn send(&self, method: &str, path: &str, value: Option<&Value>) -> Result<Response> {
        let method = match method {
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::Get,
        };
        let body = match value {
            Some(value) => Some(JsonBody::new(value)),
            None => None,
        };
        self.client
            .send(
                method,
                path,
                headers::HeaderMap::new(),
                Option::<&Value>::None,
                body,
                None,
            )
            .await
            .map_err(|e| e.into())
    }
}

impl Export for ElasticsearchExporter {
    async fn write(&self, docs: Vec<ShardDoc>) -> Result<usize> {
        let index = "metrics-shards-eshipster";
        let doc_count = docs.len();
        let ops: Vec<BulkOperation<serde_json::Value>> = docs
            .into_iter()
            .map(|doc| BulkOperation::create(doc.as_value()).into())
            .collect();

        let response = self
            .client
            .bulk(BulkParts::Index(&index))
            .body(ops)
            .send()
            .await?;

        let body = response.text().await?;
        log::trace!("{}", body);
        Ok(doc_count)
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
                log::trace!("{:?}", res);
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
