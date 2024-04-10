use std::{fs::File, io::Write};

use super::Persistor;

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
    async fn persist(&self, key: &str, bytes: axum::body::Bytes) -> Result<(), super::error::PersistError> {
        let mut file = File::open(format!("{}/{}", self.root, key))?;

        file.write_all(&bytes)?;

        Ok(())
    }
}