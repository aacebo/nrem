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
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()>;
}

pub trait Decode: Sized {
    fn decode(r: &mut impl std::io::Read) -> std::io::Result<Self>;
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Some(v) => v.encode(w),
            None => Ok(()),
        }
    }
}
