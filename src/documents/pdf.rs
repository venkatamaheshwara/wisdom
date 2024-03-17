use crate::documents::Document;
use lopdf::Document as lopdfDoc;
struct PDF {
    document: lopdfDoc,
}

impl Document for PDF {
    fn load<R>(reader: R) -> Result<Self, crate::error::Error>
    where
        Self: Sized,
        R: std::io::prelude::Read,
    {
        Ok(Self {
            document: lopdfDoc::load_from(reader)?,
        })
    }
}
