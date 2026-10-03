use crate::{Decode, Encode, Error};

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
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(&self.to_bits().to_be_bytes())?;
        Ok(())
    }
}

impl Decode for FileMode {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;

        match u32::from_be_bytes(buf) {
            0o100644 => Ok(Self::Default),
            0o040000 => Ok(Self::Tree),
            0o100755 => Ok(Self::Executable),
            0o120000 => Ok(Self::Symlink),
            0o160000 => Ok(Self::Gitlink),
            v => Err(Error::custom(format!("expected file mode, received invalid {v}"))),
        }
    }
}
