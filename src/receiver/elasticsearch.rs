use super::Receive;
use crate::client::{Auth, ElasticsearchBuilder, Host};
use crate::data::ElasticsearchApi;
use color_eyre::eyre::Result;
use elasticsearch::{http, Elasticsearch};
use serde::de::DeserializeOwned;
use url::Url;

pub struct ElasticsearchReceiver {
    client: Elasticsearch,
    url: Url,
}

impl ElasticsearchReceiver {
    pub fn new(url: Url, auth: Auth) -> Result<Self> {
        let client = ElasticsearchBuilder::new(url.clone())
            .insecure(true)
            .auth(auth)
            .build()?;

        Ok(Self { client, url })
    }

    pub fn from_host(host: Host) -> Result<Self> {
        let url = host.get_url();
        let client = ElasticsearchBuilder::from_host(host)?;
        Ok(Self { client, url })
    }
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

    async fn get<T>(&self) -> Result<T>
    where
        T: ElasticsearchApi + DeserializeOwned,
    {
        // Get the API URL path for the provided type
        let path = T::url_path();
        log::debug!("Getting API: {}", &path);

        // Send a simple GET request to the API path
        let response = self
            .client
            .send(
                http::Method::Get,
                &path,
                http::headers::HeaderMap::new(),
                Option::<&String>::None,
                Option::<&String>::None,
                None,
            )
            .await?;

        // turbo-fish serde deserialization of the JSON response
        response.json::<T>().await.map_err(Into::into)
    }
}

impl std::fmt::Display for ElasticsearchReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
