use std::os::unix::fs::{MetadataExt, PermissionsExt};

use crate::Timestamp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub ty: FileType,
    pub size: u64,
    pub executable: bool,
    pub created: Option<Timestamp>,
    pub modified: Option<Timestamp>,
}

impl From<std::fs::Metadata> for Metadata {
    fn from(meta: std::fs::Metadata) -> Self {
        Self {
            ty: meta.file_type().into(),
            size: meta.size(),
            executable: meta.permissions().mode() & 0o111 != 0,
            created: match meta.created() {
                Err(_) => None,
                Ok(v) => Some(v.into()),
            },
            modified: match meta.modified() {
                Err(_) => None,
                Ok(v) => Some(v.into()),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

impl From<std::fs::FileType> for FileType {
    fn from(ty: std::fs::FileType) -> Self {
        if ty.is_dir() {
            Self::Directory
        } else if ty.is_symlink() {
            Self::Symlink
        } else {
            Self::File
        }
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Directory => write!(f, "directory"),
            Self::Symlink => write!(f, "symlink"),
        }
    }
}
