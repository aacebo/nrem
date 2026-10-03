use std::io::{BufRead, Read};

use crate::{BufReadExt, Decode, Encode, Error, FileSystem, ObjectId};

#[allow(unused)]
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
        self.name.encode(w)?;
        w.write_all(b" -> ")?;
        self.target.encode(w)?;
        Ok(())
    }
}

impl Decode for Ref {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        let mut buf = Vec::new();
        r.read_until_consume(b' ', &mut buf)?;

        let mut lr = std::io::BufReader::new(buf.as_slice());
        let name = RefName::decode(&mut lr)?;
        r.consume_required(b"-> ")?;

        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        let mut lr = std::io::BufReader::new(buf.as_slice());
        let target = RefTarget::decode(&mut lr)?;
        Ok(Self { name, target })
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
            Self::Direct(v) => Ok(w.write_all(v.to_hex().as_bytes())?),
            Self::Symbolic(v) => {
                w.write_all(b"ref: ")?;
                v.encode(w)
            }
        }
    }
}

impl Decode for RefTarget {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);

        if r.fill_buf()?.starts_with(b"ref:") {
            r.consume_required(b"ref: ")?;
            Ok(Self::Symbolic(RefName::decode(&mut r)?))
        } else {
            let mut buf = [0u8; 64];
            r.read_exact(&mut buf)?;
            Ok(Self::Direct(ObjectId::from_hex_bytes(buf)?))
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum RefName {
    String(String),
    Bytes(Vec<u8>),
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

impl Encode for RefName {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::String(v) => Ok(w.write_all(v.as_bytes())?),
            Self::Bytes(v) => Ok(w.write_all(v.as_slice())?),
        }
    }
}

impl Decode for RefName {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        if let Ok(name) = String::from_utf8(buf.clone()) {
            Ok(Self::String(name))
        } else {
            Ok(Self::Bytes(buf))
        }
    }
}
