use std::sync::OnceLock;
use config::Config;
use serde::Deserialize;

use crate::util::log_unwrap::LogUnwrap;

static CONFIGURATION: OnceLock<ConfigWrapper> = OnceLock::new();

#[derive(Deserialize)]
struct ConfigWrapper {
    projects: Configuration,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub database_url: String,
}

impl Configuration {
    pub fn configured() -> &'static Self {
        let config = CONFIGURATION
            .get_or_init(|| {
                Config::builder()
                    .add_source(config::File::with_name("config.yaml"))
                    .add_source(config::Environment::with_prefix("PROJECTS"))
                    .build()
                    .log_unwrap()
                    .try_deserialize()
                    .log_unwrap()
            });

        &config.projects
    }
}