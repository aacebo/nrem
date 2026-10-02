use crate::{Encode, Error, FileMode, ObjectId, Timestamp};

#[derive(Debug, Clone)]
pub struct Index(Vec<IndexEntry>);

impl std::ops::Deref for Index {
    type Target = [IndexEntry];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encode for Index {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        for entry in &self.0 {
            entry.encode(w)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub path: Vec<u8>,
    pub id: ObjectId,
    pub mode: FileMode,

    pub ctime: Timestamp,
    pub mtime: Timestamp,

    pub dev: u32,
    pub ino: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,

    pub flags: IndexFlags,
}

impl Encode for IndexEntry {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut written = 0usize;

        macro_rules! write_u32 {
            ($value:expr) => {{
                w.write_all(&$value.to_be_bytes())?;
                written += 4;
            }};
        }

        write_u32!(self.ctime.secs() as u32);
        write_u32!(self.ctime.nanos());

        write_u32!(self.mtime.secs() as u32);
        write_u32!(self.mtime.nanos());

        write_u32!(self.dev);
        write_u32!(self.ino);
        write_u32!(self.mode.to_bits());
        write_u32!(self.uid);
        write_u32!(self.gid);
        write_u32!(self.size);

        self.id.encode(w)?;
        written += self.id.len();

        self.flags.encode(w)?;
        written += 2;

        w.write_all(&self.path)?;
        written += self.path.len();

        w.write_all(b"\0")?;
        written += 1;

        // v2/v3 entries are padded so each entry's total size
        // is a multiple of 8 bytes.
        let padding = (8 - (written % 8)) % 8;
        w.write_all(&b"\0".repeat(padding))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IndexStage {
    Normal,
    Base,
    Ours,
    Theirs,
}

impl TryFrom<u8> for IndexStage {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Base),
            2 => Ok(Self::Ours),
            3 => Ok(Self::Theirs),
            v => Err(Error::custom(format!("invalid index stage {v}"))),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IndexFlags {
    pub assume_valid: bool,
    pub extended: bool,
    pub stage: IndexStage,
    pub name_len: u16,
}

impl IndexFlags {
    pub fn try_from_bits(bits: u16) -> Result<Self, Error> {
        Ok(Self {
            assume_valid: bits & 0x8000 != 0,
            extended: bits & 0x4000 != 0,
            stage: IndexStage::try_from(((bits >> 12) & 0b11) as u8)?,
            name_len: bits & 0x0fff,
        })
    }

    pub fn to_bits(self) -> u16 {
        ((self.assume_valid as u16) << 15)
            | ((self.extended as u16) << 14)
            | ((self.stage as u16 & 0b11) << 12)
            | (self.name_len & 0x0fff)
    }
}

impl Encode for IndexFlags {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(&self.to_bits().to_be_bytes())
    }
}
