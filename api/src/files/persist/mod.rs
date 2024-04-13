mod error;
mod disk;

use axum::body::Bytes;
use self::{disk::DiskDrive, error::{PersistError, ReadError}};
use super::configuration::{Configuration, StorageConfiguration};

pub trait Persistor {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), PersistError>;

    async fn read(&self, key: &str) -> Result<Bytes, ReadError>;
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
    
    async fn read(&self, key: &str) -> Result<Bytes, ReadError> {
        match self {
            Self::Disk(d) => d.read(key).await,
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