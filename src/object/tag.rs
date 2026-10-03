use std::io::{BufRead, Read};

use crate::{BufReadExt, Decode, Encode, Error, ObjectRef, Signature};

#[derive(Debug, Clone)]
pub struct Tag {
    pub object: ObjectRef,
    pub name: Vec<u8>,
    pub tagger: Option<Signature>,
    pub message: Vec<u8>,
}

impl Encode for Tag {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        self.object.encode(w)?;

        w.write_all(b"\n")?;
        w.write_all(b"tag ")?;
        w.write_all(&self.name)?;
        w.write_all(b"\n")?;

        if let Some(tagger) = &self.tagger {
            w.write_all(b"tagger ")?;
            tagger.encode(w)?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"\n")?;
        w.write_all(&self.message)?;
        Ok(())
    }
}

impl Decode for Tag {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        let object = ObjectRef::decode(&mut r)?;

        r.consume_required(b"\ntag ")?;
        let mut name = Vec::new();
        r.read_until_consume(b'\n', &mut name)?;
        let mut tagger = None;

        if r.fill_buf()?.starts_with(b"tagger") {
            let mut buf = Vec::new();
            r.consume_required(b"tagger ")?;
            r.read_until_consume(b'\n', &mut buf)?;
            let mut lr = std::io::BufReader::new(buf.as_slice());
            tagger = Some(Signature::decode(&mut lr)?);
        }

        let mut message = Vec::new();
        r.consume_required(b"\n")?;
        r.read_to_end(&mut message)?;

        Ok(Self {
            object,
            name,
            tagger,
            message,
        })
    }
}
