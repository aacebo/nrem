mod indexs;
mod refs;

pub use indexs::*;
pub use refs::*;

use crate::{Error, FileSystem};

#[allow(unused)]
pub struct Repository<'a, Fs: FileSystem> {
    fs: &'a mut Fs,
    root: std::path::PathBuf,
    index: Index,
}

impl<'a, Fs: FileSystem> Repository<'a, Fs> {
    pub fn load(fs: &'a mut Fs, root: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        Ok(Self {
            fs,
            root: root.as_ref().to_path_buf(),
            index: Default::default(),
        })
    }
}
