use crate::{Encode, ObjectId};

#[derive(Clone, PartialEq, Eq)]
pub enum Link {
    Direct(ObjectId),
    Symbolic(Vec<u8>),
}

impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct(v) => write!(f, "{v}"),
            Self::Symbolic(v) => {
                if let Ok(name) = String::from_utf8(v.clone()) {
                    write!(f, "{name}")
                } else {
                    for byte in v {
                        write!(f, "{byte:02x}")?;
                    }

                    Ok(())
                }
            }
        }
    }
}

impl Encode for Link {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Direct(v) => v.encode(w),
            Self::Symbolic(v) => w.write_all(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ref {
    pub name: Vec<u8>,
    pub link: Link,
}

impl Encode for Ref {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(&self.name)?;
        w.write_all(b" -> ")?;
        self.link.encode(w)
    }
}
