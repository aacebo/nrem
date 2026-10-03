mod blob;
mod commit;
mod tag;
mod tree;

use std::io::{BufRead, Read};

pub use blob::*;
pub use commit::*;
pub use tag::*;
pub use tree::*;

use crate::{BufReadExt, Decode, Encode, Error};

#[derive(Debug, Clone)]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
    Tag(Tag),
}

impl From<Blob> for Object {
    fn from(value: Blob) -> Self {
        Self::Blob(value)
    }
}

impl From<Tree> for Object {
    fn from(value: Tree) -> Self {
        Self::Tree(value)
    }
}

impl From<Commit> for Object {
    fn from(value: Commit) -> Self {
        Self::Commit(value)
    }
}

impl From<Tag> for Object {
    fn from(value: Tag) -> Self {
        Self::Tag(value)
    }
}

impl Encode for Object {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Blob(v) => Ok(v.encode(w)?),
            Self::Tree(v) => Ok(v.encode(w)?),
            Self::Commit(v) => Ok(v.encode(w)?),
            Self::Tag(v) => Ok(v.encode(w)?),
        }
    }
}

impl Decode for Object {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);

        match r.fill_buf()? {
            v if v.starts_with(b"blob") => Ok(Blob::decode(&mut r)?.into()),
            v if v.starts_with(b"commit") => Ok(Commit::decode(&mut r)?.into()),
            v if v.starts_with(b"tag") => Ok(Tag::decode(&mut r)?.into()),
            v if v.starts_with(b"tree") => Ok(Tree::decode(&mut r)?.into()),
            _ => Err(Error::custom("expected an object")),
        }
    }
}

/// Represents the SHA-256 hash of an objects contents.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ObjectId([u8; 32]);

impl ObjectId {
    pub const fn len(self) -> usize {
        self.0.len()
    }

    pub fn from_hex_bytes(bytes: [u8; 64]) -> Result<Self, Error> {
        let mut out = [0u8; 32];

        for (index, pair) in bytes.chunks_exact(2).enumerate() {
            out[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
        }

        Ok(Self(out))
    }

    pub fn to_hex(self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }
}

impl std::fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Encode for ObjectId {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(&self.0)?;
        Ok(())
    }
}

impl Decode for ObjectId {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut buf = [0u8; 32];
        r.read_exact(&mut buf)?;
        Ok(Self(buf))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ObjectRef {
    pub id: ObjectId,
    pub ty: ObjectType,
}

impl Encode for ObjectRef {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(b"object ")?;
        w.write_all(self.id.to_hex().as_bytes())?;
        w.write_all(b"\n")?;
        w.write_all(b"type ")?;
        self.ty.encode(w)?;
        Ok(())
    }
}

impl Decode for ObjectRef {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        r.consume_required(b"object ")?;
        let mut buf = [0u8; 64];
        r.read_exact(&mut buf)?;
        let id = ObjectId::from_hex_bytes(buf)?;
        r.consume_required(b"\ntype ")?;
        let ty = ObjectType::decode(&mut r)?;
        Ok(Self { id, ty })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob => write!(f, "blob"),
            Self::Tree => write!(f, "tree"),
            Self::Commit => write!(f, "commit"),
            Self::Tag => write!(f, "tag"),
        }
    }
}

impl Decode for ObjectType {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r).take(6);
        let buf = r.fill_buf()?;

        if buf.starts_with(b"blob") {
            r.consume(4);
            Ok(Self::Blob)
        } else if buf.starts_with(b"tree") {
            r.consume(4);
            Ok(Self::Tree)
        } else if buf.starts_with(b"commit") {
            r.consume(6);
            Ok(Self::Commit)
        } else if buf.starts_with(b"tag") {
            r.consume(3);
            Ok(Self::Tag)
        } else {
            Err(Error::custom("expected object type"))
        }
    }
}

impl Encode for ObjectType {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Blob => Ok(w.write_all(b"blob")?),
            Self::Tree => Ok(w.write_all(b"tree")?),
            Self::Commit => Ok(w.write_all(b"commit")?),
            Self::Tag => Ok(w.write_all(b"tag")?),
        }
    }
}

fn hex_nibble(byte: u8) -> Result<u8, Error> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(Error::custom("invalid hexadecimal object ID")),
    }
}
