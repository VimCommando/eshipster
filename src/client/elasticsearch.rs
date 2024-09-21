use base64::{engine::general_purpose::STANDARD, Engine};
use clap::ValueEnum;
use color_eyre::eyre::Result;
use elasticsearch::{
    self,
    cert::CertificateValidation,
    http::{
        self,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
};
use std::str::FromStr;
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

    pub fn apikey(self, apikey: String) -> Self {
        let mut headers = self.headers;
        headers.append(
            http::headers::AUTHORIZATION,
            format!("ApiKey {}", apikey)
                .parse()
                .expect("Invalid API key"),
        );
        Self { headers, ..self }
    }

    pub fn auth(self, auth: Auth) -> Self {
        log::debug!("Setting client auth to {}", auth);
        match auth {
            Auth::Apikey(apikey) => self.apikey(apikey),
            Auth::Basic(username, password) => self.basic_auth(username, password),
            Auth::None => self,
        }
    }

    pub fn basic_auth(self, username: String, password: String) -> Self {
        let mut headers = self.headers;
        headers.append(
            http::headers::AUTHORIZATION,
            http::headers::HeaderValue::from_str(&format!(
                "Basic {}",
                STANDARD.encode(&format!("{}:{}", username, password))
            ))
            .expect("Invalid basic auth"),
        );
        Self { headers, ..self }
    }

    pub fn build(self) -> Result<elasticsearch::Elasticsearch> {
        let transport = TransportBuilder::new(self.connection_pool)
            .headers(self.headers)
            .cert_validation(self.cert_validation)
            .build()?;
        Ok(elasticsearch::Elasticsearch::new(transport))
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum AuthType {
    Apikey,
    Basic,
    None,
}

impl FromStr for AuthType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "apikey" => Ok(Self::Apikey),
            "basic" => Ok(Self::Basic),
            "none" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

pub enum Auth {
    Apikey(String),
    Basic(String, String),
    None,
}

impl Auth {
    pub fn new(
        r#type: &AuthType,
        username: Option<String>,
        password: Option<String>,
        apikey: Option<String>,
    ) -> Self {
        match (r#type, username, password, apikey) {
            (AuthType::Apikey, _, _, Some(apikey)) => Self::Apikey(apikey),
            (AuthType::Basic, Some(username), Some(password), _) => Self::Basic(username, password),
            (AuthType::None, _, _, _) | _ => Self::None,
        }
    }
}

impl std::fmt::Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apikey(_) => write!(f, "Apikey"),
            Self::Basic(_, _) => write!(f, "Basic"),
            Self::None => write!(f, "None"),
        }
    }
}
