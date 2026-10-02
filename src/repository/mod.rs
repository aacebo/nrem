mod index;
mod link;

pub use index::*;
pub use link::*;

use crate::{Error, FileSystem};

pub struct Repository<Fs: FileSystem> {
    fs: Fs,
    root: std::path::PathBuf,
    index: Index,
}

impl<Fs: FileSystem> Repository<Fs> {
    pub fn load(fs: Fs, root: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        Ok(Self {
            fs,
            root: root.as_ref().to_path_buf(),
            index: Default::default(),
        })
    }
}
