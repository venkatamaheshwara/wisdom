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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn pdf_load() {
        let dummy_pdf = "test/artifacts/dummy.pdf";
        let file_reader = fs::OpenOptions::new().read(true).open(dummy_pdf).unwrap();
        let pdf_doc = PDF::load(file_reader).unwrap();
        let text = pdf_doc.document.extract_text(&[1]).unwrap();
        assert_eq!("Hello World\n", text);
    }
}
