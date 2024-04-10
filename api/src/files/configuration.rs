use std::sync::OnceLock;
use config::Config;
use serde::Deserialize;

use crate::util::log_unwrap::LogUnwrap;

static CONFIGURATION: OnceLock<ConfigWrapper> = OnceLock::new();

#[derive(Deserialize)]
struct ConfigWrapper {
    files: Configuration,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub database_url: String,
    #[serde(flatten)]
    pub driver: StorageConfiguration,
}

#[derive(Deserialize)]
#[serde(tag = "driver")]
#[serde(rename_all = "lowercase")]
pub enum StorageConfiguration {
    Disk { 
        path: String 
    },

    S3 {
        bucket_name: String,
    }
}

impl Configuration {
    pub fn configured() -> &'static Self {
        let config = CONFIGURATION
            .get_or_init(|| {
                Config::builder()
                    .add_source(config::File::with_name("Config.yaml"))
                    .add_source(config::Environment::with_prefix("FILES"))
                    .build()
                    .log_unwrap()
                    .try_deserialize()
                    .log_unwrap()
            });

        &config.files
    }
}