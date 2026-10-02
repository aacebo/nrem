use crate::{Encode, ObjectId, TimestampTz};

#[derive(Debug, Clone)]
pub struct Commit {
    /// Root filesystem/tree snapshot.
    pub tree_id: ObjectId,

    /// Additional headers may exist.
    ///
    /// Examples:
    ///
    ///     encoding
    ///     gpgsig
    ///     mergetag
    ///     tree-related extensions
    pub headers: Vec<Header>,

    /// Usually one parent.
    ///
    /// Initial commit:
    ///     0 parents
    ///
    /// Normal commit:
    ///     1 parent
    ///
    /// Merge commit:
    ///     2+ parents
    pub parents: Vec<ObjectId>,
    pub author: Signature,
    pub committer: Signature,
    pub message: Vec<u8>,
}

impl Encode for Commit {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(b"tree ")?;
        self.tree_id.encode(w)?;
        w.write_all(b"\n")?;

        for parent in &self.parents {
            parent.encode(w)?;
            w.write_all(b"\n")?;
        }

        self.author.encode(w)?;
        w.write_all(b"\n")?;

        self.committer.encode(w)?;
        w.write_all(b"\n")?;

        for header in &self.headers {
            header.encode(w)?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"\n")?;
        w.write_all(&self.message)
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: Vec<u8>,
}

impl Encode for Header {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(self.name.as_bytes())?;
        w.write_all(b" ")?;
        w.write_all(&self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub time: TimestampTz,
}

impl Encode for Signature {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(self.name.as_bytes())?;
        w.write_all(b" ")?;
        w.write_all(self.email.as_bytes())?;
        w.write_all(b" ")?;
        w.write_all(&self.time.secs().to_be_bytes())?;
        w.write_all(b" ")?;
        w.write_all(&self.time.offset().to_be_bytes())
    }
}
