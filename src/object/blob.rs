use std::io::Read;

use crate::{BufReadExt, Decode, Encode, Error};

/// Blob
///
/// Byte structure:
///
///     HEADER:
///         b"blob "
///         decimal content length
///         0x00
///
///     CONTENT:
///         arbitrary bytes
///
/// Example:
///
///     content = b"hello"
///
/// Serialized:
///
///     62 6c 6f 62 20 35 00 68 65 6c 6c 6f
///     b  l  o  b     5 \0 h  e  l  l  o
///
/// i.e.
///
///     b"blob 5\0hello"
///
#[derive(Clone, PartialEq, Eq)]
pub struct Blob(Vec<u8>);

impl std::ops::Deref for Blob {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blob<{}>", self.0.len())
    }
}

impl Encode for Blob {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        w.write_all(b"blob ")?;
        write!(w, "{}", self.0.len())?;
        w.write_all(b"\0")?;
        w.write_all(&self.0)?;
        Ok(())
    }
}

impl Decode for Blob {
    fn decode(r: &mut impl std::io::BufRead) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        r.consume_required(b"blob ")?;

        let mut buf = Vec::new();
        r.read_until_consume(b'\0', &mut buf)?;

        let text = std::str::from_utf8(&buf).map_err(|_| Error::custom("length is not UTF-8"))?;
        let len: u32 = text.parse().map_err(|_| Error::custom("invalid length"))?;
        let mut data = vec![0; len as usize];
        r.read_exact(&mut data)?;
        Ok(Self(data))
    }
}
