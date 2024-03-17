use crate::error::Error;
use std::io::Read;
pub(crate) trait Document {
    fn load<R>(reader: R) -> Result<Self, Error>
    where
        Self: Sized,
        R: Read;
}

mod pdf;
