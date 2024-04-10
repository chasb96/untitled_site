mod error;
mod disk;

use axum::body::Bytes;
use self::{error::PersistError, disk::DiskDrive};
use super::configuration::{Configuration, StorageConfiguration};

pub trait Persistor {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), PersistError>;
}

pub enum PersistorOption<'a> {
    Disk(DiskDrive<'a>)
}

impl<'a> Persistor for PersistorOption<'a> {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), PersistError> {
        match self {
            Self::Disk(d) => d.persist(key, bytes).await
        }
    }
}

impl<'a> Default for PersistorOption<'a> {
    fn default() -> Self {
        let configuration = Configuration::configured();

        match &configuration.driver {
            StorageConfiguration::Disk { path } => PersistorOption::Disk(DiskDrive::new(path)),
            StorageConfiguration::S3 { bucket_name: _ } => unimplemented!(),
        }
    }
}