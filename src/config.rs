use dotenvy::dotenv;
use std::sync::LazyLock;

pub fn load() {
    // Load environment variables from .env file
    dotenv().ok();
}

pub static LOG_LEVEL: &str = "info";

type Setting = LazyLock<Option<String>>;

pub static ESHIPSTER_RC_USERNAME: Setting =
    LazyLock::new(|| std::env::var("ESHIPSTER_RC_USERNAME").ok());
pub static ESHIPSTER_RC_PASSWORD: Setting =
    LazyLock::new(|| std::env::var("ESHIPSTER_RC_PASSWORD").ok());
pub static ESHIPSTER_RC_APIKEY: Setting =
    LazyLock::new(|| std::env::var("ESHIPSTER_RC_APIKEY").ok());
pub static ESHIPSTER_XP_USERNAME: Setting =
    LazyLock::new(|| std::env::var("ESHIPSTER_XP_USERNAME").ok());
pub static ESHIPSTER_XP_PASSWORD: Setting =
    LazyLock::new(|| std::env::var("ESHIPSTER_XP_PASSWORD").ok());
pub static ESHIPSTER_XP_APIKEY: Setting =
    LazyLock::new(|| std::env::var("ESHIPSTER_XP_APIKEY").ok());
