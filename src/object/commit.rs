use std::io::{BufRead, Read};

use crate::{BufReadExt, Decode, Encode, Error, ObjectId, Timestamp, TimestampTz};

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
        w.write_all(self.tree_id.to_hex().as_bytes())?;
        w.write_all(b"\n")?;

        for parent in &self.parents {
            w.write_all(b"parent ")?;
            w.write_all(parent.to_hex().as_bytes())?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"author ")?;
        self.author.encode(w)?;
        w.write_all(b"\n")?;

        w.write_all(b"committer ")?;
        self.committer.encode(w)?;
        w.write_all(b"\n")?;

        for header in &self.headers {
            header.encode(w)?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"\n")?;
        w.write_all(&self.message)?;
        Ok(())
    }
}

impl Decode for Commit {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        let mut buf = [0u8; 64];
        r.consume_required(b"tree ")?;
        r.read_exact(&mut buf)?;

        let tree_id = ObjectId::from_hex_bytes(buf)?;
        r.consume_required(b"\n")?;

        let mut parents = Vec::new();

        while r.fill_buf()?.starts_with(b"parent") {
            let mut buf = [0u8; 64];
            r.consume_required(b"parent ")?;
            r.read_exact(&mut buf)?;
            parents.push(ObjectId::from_hex_bytes(buf)?);
            r.consume_required(b"\n")?;
        }

        let mut buf = Vec::new();
        r.consume_required(b"author ")?;
        r.read_until_consume(b'\n', &mut buf)?;
        let mut lr = std::io::BufReader::new(buf.as_slice());
        let author = Signature::decode(&mut lr)?;

        r.consume_required(b"committer ")?;
        let mut buf = Vec::new();
        r.read_until_consume(b'\n', &mut buf)?;
        let mut lr = std::io::BufReader::new(buf.as_slice());
        let committer = Signature::decode(&mut lr)?;
        let mut headers = Vec::new();

        while !r.fill_buf()?.starts_with(b"\n") {
            let mut buf = Vec::new();
            r.read_until_consume(b'\n', &mut buf)?;
            let mut lr = std::io::BufReader::new(buf.as_slice());
            headers.push(Header::decode(&mut lr)?);
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

impl Decode for Header {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        let mut buf = Vec::new();

        r.read_until_consume(b' ', &mut buf)?;

        let name = String::from_utf8(buf)?;
        let mut value = Vec::new();

        r.read_to_end(&mut value)?;
        Ok(Self { name, value })
    }
}

impl Encode for Header {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(self.name.as_bytes())?;
        w.write_all(b" ")?;
        w.write_all(&self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub time: TimestampTz,
}

impl Encode for Signature {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(self.name.as_bytes())?;
        w.write_all(b" ")?;
        w.write_all(self.email.as_bytes())?;
        w.write_all(b" ")?;
        w.write_all(&self.time.secs().to_be_bytes())?;
        w.write_all(b" ")?;
        w.write_all(&self.time.offset().to_be_bytes())?;
        Ok(())
    }
}

impl Decode for Signature {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
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
