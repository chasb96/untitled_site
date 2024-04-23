use std::{env, sync::OnceLock};
use log::error;
use serde::Deserialize;

use crate::util::log_unwrap::LogUnwrap;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

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
                let database_url = env::var("FILES_DATABASE_URL").log_unwrap();

                let driver = match env::var("FILES_DRIVER").log_unwrap().to_lowercase().as_str() {
                    "disk" => StorageConfiguration::Disk { 
                        path: env::var("FILES_PATH").log_unwrap() 
                    },
                    "s3" => StorageConfiguration::S3 { 
                        bucket_name: env::var("FILES_BUCKET_NAME").log_unwrap() 
                    },
                    driver => {
                        error!("Invalid storage driver {}", driver); 
                        panic!("Invalid storage driver {}", driver);
                    }
                };

                Configuration {
                    database_url,
                    driver,
                }
            });

        &config
    }
}