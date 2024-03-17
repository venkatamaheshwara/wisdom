use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::connectors::Connector;
use crate::documents::Document;
use crate::error::Error;
pub struct File<T>
where
    T: Document,
{
    path: PathBuf,
    phantomdata: PhantomData<T>,
}

impl<T> File<T>
where
    T: Document,
{
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into(),
            phantomdata: PhantomData::default(),
        }
    }
    fn read(&self) -> Result<T, Error> {
        let file_reader = fs::OpenOptions::new().read(true).open(&self.path)?;
        Ok(T::load(file_reader)?)
    }
}

impl<T> Connector for File<T>
where
    T: Document,
{
    type Doc = T;
    fn poll(&self) -> Result<Self::Doc, Error> {
        self.read()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Doc(String);
    impl Document for Doc {
        fn load<R>(mut reader: R) -> Result<Self, Error>
        where
            Self: Sized,
            R: std::io::prelude::Read,
        {
            let mut s = String::new();
            reader.read_to_string(&mut s)?;
            Ok(Self(s))
        }
    }

    #[test]
    fn test_read() {
        // Refer to Cargo.toml
        let file: File<Doc> = File::new("Cargo.toml");
        let doc = file.poll().unwrap();
        assert!(doc.0.starts_with("[package]\nname = \"wisdom\""))
    }
}
