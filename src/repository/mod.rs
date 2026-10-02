mod index;
mod link;

pub use index::*;
pub use link::*;

use crate::FileSystem;

pub struct Repository<Fs: FileSystem> {
    #[allow(unused)]
    fs: Fs,
}

impl<Fs: FileSystem> Repository<Fs> {
    pub fn new(fs: Fs) -> Self {
        Self { fs }
    }
}
