use crate::Encode;

/// Unix-style mode encoded as ASCII in the tree object.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileMode {
    /// normal default file
    Default = 100644,

    /// executable
    Executable = 100755,

    /// symbolic link
    Symlink = 120000,

    /// tree/directory
    Tree = 40000,

    /// gitlink/submodule
    Gitlink = 160000,
}

impl FileMode {
    pub const fn to_bits(self) -> u32 {
        match self {
            Self::Default => 0o100644,
            Self::Tree => 0o040000,
            Self::Executable => 0o100755,
            Self::Symlink => 0o120000,
            Self::Gitlink => 0o160000,
        }
    }
}

impl std::fmt::Display for FileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Encode for FileMode {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Default => w.write_all(b"100644"),
            Self::Executable => w.write_all(b"100755"),
            Self::Symlink => w.write_all(b"120000"),
            Self::Tree => w.write_all(b"040000"),
            Self::Gitlink => w.write_all(b"160000"),
        }
    }
}
