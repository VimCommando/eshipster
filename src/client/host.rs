use color_eyre::eyre::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

/// Get the path for the hosts.yml file, fallback to ~/.eshipster/hosts.yml
pub fn get_hosts_path() -> PathBuf {
    match env::var("ESHIPSTER_HOSTS") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let home = match env::var("HOME") {
                Ok(home) => PathBuf::from(home),
                Err(_) => panic!("ERROR: No home directory found"),
            };
            // Check if the `.eshipster` directory exists, if not, create it
            let eshipster_dir = home.join(".eshipster");
            if !eshipster_dir.exists() {
                create_dir(&eshipster_dir).expect("Failed to create ~/.eshipster directory");
            }
            let path = home.join(".eshipster").join("hosts.yml");
            path
        }
    }
}

/// Loads hosts from a yml file
pub fn parse_hosts_yml() -> Result<BTreeMap<String, Host>> {
    let path = get_hosts_path();
    log::debug!("Parsing {:?}", path);
    let hosts = match path.is_file() {
        true => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let hosts: Result<BTreeMap<String, Host>, serde_yaml::Error> =
                serde_yaml::from_reader(reader);
            hosts
        }
        false => {
            log::info!("No hosts, file creating {:?}", path);
            File::create(path)?;
            Ok(BTreeMap::new())
        }
    };
    Ok(hosts?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "auth")]
pub enum Host {
    ApiKey {
        insecure: Option<bool>,
        apikey: String,
        url: Url,
    },
    Basic {
        insecure: Option<bool>,
        password: String,
        url: Url,
        username: String,
    },
    None {
        insecure: Option<bool>,
        url: Url,
    },
}

impl Host {
    pub fn parse(host: &str) -> Option<Self> {
        // parse the ~/.eshipster/hosts.yml file into a HashMap<String, Host>
        let hosts = match parse_hosts_yml() {
            Ok(hosts) => hosts,
            Err(e) => {
                log::error!("Error parsing hosts.yml: {}", e);
                return None;
            }
        };
        log::debug!(
            "Known hosts: {}",
            hosts
                .clone()
                .into_iter()
                .map(|(k, _)| k)
                .collect::<Vec<String>>()
                .join(", ")
        );
        hosts.get(host).cloned()
    }

    pub fn get_url(&self) -> Url {
        match self {
            Self::ApiKey { url, .. } => url.clone(),
            Self::Basic { url, .. } => url.clone(),
            Self::None { url, .. } => url.clone(),
        }
    }

    pub async fn test(&self) -> Result<bool> {
        match self {
            Self::ApiKey {
                apikey,
                insecure,
                url,
            } => {
                // test the connection
                log::info!("Testing connection: {}", &url);
                // create a client with the API key
                let client = reqwest::Client::builder()
                    .default_headers(
                        std::iter::once((
                            reqwest::header::AUTHORIZATION,
                            format!("ApiKey {}", apikey)
                                .parse()
                                .expect("Failed to parse apikey"),
                        ))
                        .collect(),
                    )
                    .danger_accept_invalid_certs(insecure.unwrap_or(false))
                    .build()?;
                log::trace!("Reqwest client: {:?}", client);
                let response = client.get(url.as_str()).send().await;
                match response {
                    Ok(response) => Ok(response.status().is_success()),
                    Err(e) => Err(e.into()),
                }
            }
            Self::Basic {
                insecure,
                password,
                url,
                username,
            } => {
                // test the connection
                log::info!("Testing connection: {}", &url);
                let client = reqwest::Client::builder()
                    .danger_accept_invalid_certs(insecure.unwrap_or(false))
                    .build()?;
                let response = client
                    .get(url.as_str())
                    .basic_auth(username, Some(password))
                    .send()
                    .await;
                match response {
                    Ok(response) => Ok(response.status().is_success()),
                    Err(e) => Err(e.into()),
                }
            }
            Self::None { url, .. } => {
                // test the connection
                log::info!("Testing connection {}", &url);
                let response = reqwest::get(url.as_str()).await;
                match response {
                    Ok(response) => Ok(response.status().is_success()),
                    Err(e) => Err(e.into()),
                }
            }
        }
    }
}

impl Display for Host {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiKey { url, .. } => write!(fmt, "Host ApiKey: {}", url,),
            Self::Basic { url, username, .. } => write!(fmt, "Host Basic: {}@ {}", username, url,),
            Self::None { url, .. } => write!(fmt, "Host None: {}", url),
        }
    }
}

impl FromStr for Host {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Host::parse(&s.to_string()) {
            Some(host) => Ok(host),
            None => Err(()),
        }
    }
}
