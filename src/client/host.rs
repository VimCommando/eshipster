use color_eyre::eyre::Result;
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

/// Get the path for the hosts.yml file, fallback to ~/.eshipster/hosts.yml
fn get_hosts_path() -> Result<PathBuf> {
    match env::var("ESHIPSTER_HOSTS") {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => {
            let home = env::var("HOME").map(|home| PathBuf::from(home))?;
            // Check if the `.eshipster` directory exists, if not, create it
            let eshipster_dir = home.join(".eshipster");
            if !eshipster_dir.exists() {
                create_dir(&eshipster_dir)?
            }
            let path = home.join(".eshipster").join("hosts.yml");
            Ok(path)
        }
    }
}

/// Tries to load hosts from a yml file, creates an empty file if it doesn't exist
fn parse_hosts_yml() -> Result<BTreeMap<String, Host>> {
    let path = get_hosts_path()?;
    log::debug!("Parsing {:?}", path);
    match path.is_file() {
        true => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let hosts: BTreeMap<String, Host> = serde_yaml::from_reader(reader)?;
            Ok(hosts)
        }
        false => {
            log::info!("No hosts, file creating {:?}", path);
            File::create(path)?;
            Ok(BTreeMap::new())
        }
    }
}
