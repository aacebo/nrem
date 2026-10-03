use crate::{Encode, Error, FileSystem, ObjectId};

pub struct Refs<'a, Fs: FileSystem> {
    fs: &'a Fs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ref {
    pub name: RefName,
    pub target: RefTarget,
}

impl Encode for Ref {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(self.name.as_bytes())?;
        w.write_all(b" -> ")?;
        self.target.encode(w)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum RefName {
    String(String),
    Bytes(Vec<u8>),
}

impl RefName {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        if let Ok(name) = String::from_utf8(bytes.clone()) {
            Self::String(name)
        } else {
            Self::Bytes(bytes)
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::String(v) => v.as_bytes(),
            Self::Bytes(v) => v.as_slice(),
        }
    }
}

impl std::fmt::Debug for RefName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for RefName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{v}"),
            Self::Bytes(v) => {
                for byte in v {
                    write!(f, "{byte:02x}")?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum RefTarget {
    Direct(ObjectId),
    Symbolic(RefName),
}

impl std::fmt::Debug for RefTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for RefTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct(v) => write!(f, "{v}"),
            Self::Symbolic(v) => write!(f, "{v}"),
        }
    }
}

impl Encode for RefTarget {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Direct(v) => Ok(v.encode(w)?),
            Self::Symbolic(v) => Ok(w.write_all(v.as_bytes())?),
        }
    }
}
