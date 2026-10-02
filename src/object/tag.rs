use crate::{Encode, ObjectRef, Signature};

#[derive(Debug, Clone)]
pub struct Tag {
    pub object: ObjectRef,
    pub name: Vec<u8>,
    pub tagger: Option<Signature>,
    pub message: Vec<u8>,
}

impl Encode for Tag {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        self.object.encode(w)?;

        w.write_all(b"tag ")?;
        w.write_all(&self.name)?;
        w.write_all(b"\n")?;

        if let Some(tagger) = &self.tagger {
            w.write_all(b"tagger ")?;
            tagger.encode(w)?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"\n")?;
        w.write_all(&self.message)
    }
}
