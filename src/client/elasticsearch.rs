use color_eyre::eyre::Result;
use elasticsearch::{
    self,
    cert::CertificateValidation,
    http::{
        self,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
};
use url::Url;

pub struct ElasticsearchBuilder {
    cert_validation: CertificateValidation,
    connection_pool: SingleNodeConnectionPool,
    headers: http::headers::HeaderMap,
    url: Url,
}

impl ElasticsearchBuilder {
    pub fn new(url: Url) -> Self {
        let mut headers = http::headers::HeaderMap::new();
        headers.append(http::headers::ACCEPT_ENCODING, "gzip".parse().unwrap());

        Self {
            cert_validation: CertificateValidation::Default,
            connection_pool: SingleNodeConnectionPool::new(url.clone()),
            headers,
            url,
        }
    }

    pub fn insecure(self, ignore_certs: bool) -> Self {
        let cert_validation = match ignore_certs {
            true => CertificateValidation::None,
            false => CertificateValidation::Default,
        };
        Self {
            cert_validation,
            ..self
        }
    }

    pub fn build(self) -> Result<elasticsearch::Elasticsearch> {
        let transport = TransportBuilder::new(self.connection_pool)
            .headers(self.headers)
            .cert_validation(self.cert_validation)
            .build()?;
        Ok(elasticsearch::Elasticsearch::new(transport))
    }
}
