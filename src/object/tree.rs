use crate::{BufReadExt, Decode, Encode, Error, FileMode, ObjectId};

#[derive(Debug, Clone)]
pub struct Tree(Vec<TreeEntry>);

impl std::ops::Deref for Tree {
    type Target = [TreeEntry];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encode for Tree {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        for entry in &self.0 {
            entry.encode(w)?;
        }

        Ok(())
    }
}

impl Decode for Tree {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut entries = Vec::new();

        while let Ok(entry) = TreeEntry::decode(r) {
            entries.push(entry);
        }

        Ok(Self(entries))
    }
}

#[derive(Clone)]
pub struct TreeEntry {
    /// Object referenced by this entry.
    pub id: ObjectId,

    /// Raw filename bytes in actual Git.
    ///
    /// Git does not require UTF-8 filenames.
    pub name: Vec<u8>,

    /// Unix-style mode encoded as ASCII in the tree object.
    pub mode: FileMode,
}

impl std::fmt::Debug for TreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("TreeEntry");
        s.field("id", &self.id);

        if let Ok(name) = String::from_utf8(self.name.clone()) {
            s.field("name", &name);
        } else {
            s.field("name", &self.name.iter().map(|b| format!("{b:02x}")).collect::<String>());
        }

        s.finish()
    }
}

impl Encode for TreeEntry {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        self.mode.encode(w)?;
        w.write_all(b" ")?;
        w.write_all(&self.name)?;
        w.write_all(b"\0")?;
        self.id.encode(w)?;
        Ok(())
    }
}

impl Decode for TreeEntry {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut r = std::io::BufReader::new(r);
        let mode = FileMode::decode(&mut r)?;

        r.consume_required(b" ")?;

        let mut name = Vec::new();
        r.read_until_consume(b'\0', &mut name)?;

        let id = ObjectId::decode(&mut r)?;
        Ok(Self { id, name, mode })
    }
}
