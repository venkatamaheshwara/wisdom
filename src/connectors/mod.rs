use crate::documents::Document;
use crate::error::Error;
mod file;
pub(crate) trait Connector {
    type Doc: Document;
    fn poll(&self) -> Result<Self::Doc, Error>;
}

pub use file::File;
