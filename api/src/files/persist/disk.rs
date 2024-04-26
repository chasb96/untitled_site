use std::{fs::File, io::{Read, Write}};
use bytes::Bytes;
use super::{error::ReadError, Persistor};

pub struct DiskDrive<'a> {
    root: &'a String,
}

impl<'a> DiskDrive<'a> {
    pub fn new(root: &'a String) -> Self {
        Self {
            root
        }
    }
}

impl<'a> Persistor for DiskDrive<'a> {
    async fn persist(&self, key: &str, bytes: Bytes) -> Result<(), super::error::PersistError> {
        let mut file = File::create(format!("{}/{}", self.root, key))?;

        file.write_all(&bytes)?;

        Ok(())
    }
    
    async fn read(&self, key: &str) -> Result<Bytes, ReadError> {
        let mut file = File::open(format!("{}/{}", self.root, key))?;

        let mut buf = Vec::new();

        file.read_to_end(&mut buf)?;

        Ok(Bytes::from(buf))
    }
}