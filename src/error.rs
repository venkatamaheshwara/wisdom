use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Other Error: {0}")]
    Other(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Other(format!("{:?}", value))
    }
}
impl From<lopdf::Error> for Error {
    fn from(value: lopdf::Error) -> Self {
        Self::Other(format!("{:?}", value))
    }
}
