use std::io::{BufRead, Read};

use crate::{BufReadExt, Decode, Encode, Error, FromBytes, ObjectId, Timestamp, TimestampTz, ToBytes};

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
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(b"tree ")?;
        self.tree_id.encode(w)?;
        w.write_all(b"\n")?;

        for parent in &self.parents {
            w.write_all(b"parent ")?;
            parent.encode(w)?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"author ")?;
        w.write_all(&self.author.to_bytes())?;
        w.write_all(b"\n")?;

        w.write_all(b"committer ")?;
        w.write_all(&self.committer.to_bytes())?;
        w.write_all(b"\n")?;

        for header in &self.headers {
            w.write_all(&header.to_bytes())?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"\n")?;
        w.write_all(&self.message)?;
        Ok(())
    }
}

impl Decode for Commit {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        r.consume_required(b"tree ")?;

        let tree_id = ObjectId::decode(&mut r)?;
        r.consume_required(b"\n")?;

        let mut parents = Vec::new();

        while r.fill_buf()?.starts_with(b"parent") {
            r.consume_required(b"parent ")?;
            parents.push(ObjectId::decode(&mut r)?);
            r.consume_required(b"\n")?;
        }

        let mut buf = Vec::new();
        r.read_until_consume(b' ', &mut buf)?;

        if &buf != b"author" {
            return Err(Error::custom("expected `author`"));
        }

        let mut buf = Vec::new();
        r.read_until_consume(b'\n', &mut buf)?;

        let author = Signature::from_bytes(&buf)?;
        let mut buf = Vec::new();
        r.read_until_consume(b' ', &mut buf)?;

        if &buf != b"committer" {
            return Err(Error::custom("expected `committer`"));
        }

        let mut buf = Vec::new();
        r.read_until_consume(b'\n', &mut buf)?;

        let committer = Signature::from_bytes(&buf)?;
        let mut headers = Vec::new();

        while !r.fill_buf()?.starts_with(b"\n") {
            let mut buf = Vec::new();
            r.read_until_consume(b'\n', &mut buf)?;
            headers.push(Header::from_bytes(&buf)?);
        }

        let mut message = Vec::new();

        r.consume_required(b"\n")?;
        r.read_to_end(&mut message)?;

        Ok(Self {
            tree_id,
            headers,
            parents,
            author,
            committer,
            message,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: Vec<u8>,
}

impl FromBytes for Header {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(bytes);
        let mut buf = Vec::new();

        r.read_until_consume(b' ', &mut buf)?;

        let name = String::from_utf8(buf)?;
        let mut value = Vec::new();

        r.read_to_end(&mut value)?;
        Ok(Self { name, value })
    }
}

impl ToBytes for Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(&self.value);
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub time: TimestampTz,
}

impl FromBytes for Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(bytes);
        let mut buf = Vec::new();

        r.read_until_consume(b' ', &mut buf)?;

        let name = String::from_utf8(buf)?;
        let mut buf = Vec::new();

        r.read_until_consume(b' ', &mut buf)?;

        let email = String::from_utf8(buf)?;
        let mut buf = [0u8; 8];

        r.read_exact(&mut buf)?;
        r.consume_required(b" ")?;

        let seconds = i64::from_be_bytes(buf);
        let mut buf = [0u8; 4];

        r.read_exact(&mut buf)?;
        let offset = i32::from_be_bytes(buf);

        Ok(Self {
            name,
            email,
            time: TimestampTz::new(Timestamp::new(seconds, 0), offset),
        })
    }
}

impl ToBytes for Signature {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(self.email.as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(&self.time.secs().to_be_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(&self.time.offset().to_be_bytes());
        bytes
    }
}
