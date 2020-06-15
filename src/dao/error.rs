use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DBError {
    message: String,
    source: tokio_postgres::Error,
}

impl DBError {
    pub fn new(message: &str, source: tokio_postgres::Error) -> DBError {
        DBError {
            message: message.to_owned(),
            source: source,
        }
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DBError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}