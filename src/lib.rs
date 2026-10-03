mod error;
mod fs;
mod object;
mod repository;
mod time;

pub use error::*;
pub use fs::*;
pub use object::*;
pub use repository::*;
pub use time::*;

pub trait Encode {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error>;
}

pub trait Decode: Sized {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, Error>;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>;
}

pub trait BufReadExt: std::io::BufRead {
    fn consume_required(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        if !self.fill_buf()?.starts_with(bytes) {
            let message = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
            return Err(std::io::Error::other(Error::custom(format!("expected `{message}`"))));
        }

        Ok(())
    }

    fn read_until_consume(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let size = self.read_until(byte, buf)?;
        self.consume(1);
        Ok(size + 1)
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, w: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Some(v) => v.encode(w),
            None => Ok(()),
        }
    }
}

impl<T: std::io::BufRead> BufReadExt for T {}
