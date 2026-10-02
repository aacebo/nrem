mod blob;
mod commit;
mod tag;
mod tree;

pub use blob::*;
pub use commit::*;
pub use tag::*;
pub use tree::*;

use crate::Encode;

/// Represents the SHA-256 hash of an objects contents.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ObjectId([u8; 32]);

impl ObjectId {
    pub const fn len(self) -> usize {
        self.0.len()
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
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(&self.0)
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

impl Encode for ObjectType {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Blob => w.write_all(b"blob"),
            Self::Tree => w.write_all(b"tree"),
            Self::Commit => w.write_all(b"commit"),
            Self::Tag => w.write_all(b"tag"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ObjectRef {
    pub id: ObjectId,
    pub ty: ObjectType,
}

impl Encode for ObjectRef {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(b"object ")?;
        self.id.encode(w)?;
        w.write_all(b"\n")?;

        w.write_all(b"type ")?;
        self.ty.encode(w)?;
        w.write_all(b"\n")
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
    Tag(Tag),
}

impl Encode for Object {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Blob(v) => v.encode(w),
            Self::Tree(v) => v.encode(w),
            Self::Commit(v) => v.encode(w),
            Self::Tag(v) => v.encode(w),
        }
    }
}
