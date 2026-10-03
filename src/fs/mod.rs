mod meta;
mod mode;

pub use meta::*;
pub use mode::*;

use crate::Error;

pub trait FileSystem {
    fn read(&self, path: &std::path::Path) -> Result<Vec<u8>, Error>;
    fn metadata(&self, path: &std::path::Path) -> Result<Metadata, Error>;
    fn write(&mut self, path: &std::path::Path, bytes: &[u8]) -> Result<(), Error>;
    fn remove(&mut self, path: &std::path::Path) -> Result<(), Error>;
}

pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn read(&self, path: &std::path::Path) -> Result<Vec<u8>, Error> {
        Ok(std::fs::read(path)?)
    }

    fn metadata(&self, path: &std::path::Path) -> Result<Metadata, Error> {
        Ok(std::fs::metadata(path)?.into())
    }

    fn write(&mut self, path: &std::path::Path, bytes: &[u8]) -> Result<(), Error> {
        Ok(std::fs::write(path, bytes)?)
    }

    fn remove(&mut self, path: &std::path::Path) -> Result<(), Error> {
        match self.metadata(path)?.ty {
            FileType::Directory => Ok(std::fs::remove_dir(path)?),
            FileType::File | FileType::Symlink => Ok(std::fs::remove_file(path)?),
        }
    }
}
