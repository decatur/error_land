pub mod error_string;
mod loc_error;
mod loc_formater;

use core::fmt;

pub use loc_error::{CoreError, StackItem};
pub use loc_formater::{JsonFormatter, PrettyFormatter};

#[derive(Debug)]
struct Error(String);

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

pub fn into_err(msg: impl Into<String>) -> impl std::error::Error {
    Error::new(msg)
}

#[cfg(test)]
mod tests {
    use crate::{err_struct, into_err};

    err_struct!(ErrorA);
    fn a() -> Result<(), ErrorA> {
        Err(into_err("fn a() did bad"))?
    }

    #[test]
    fn test() {
        let e = a().err().unwrap();
        assert_eq!(e.inner.len(), 1);
        assert_eq!(e.inner[0].msg, "fn a() did bad");
        assert_eq!(e.inner[0].location, "src/lib.rs:37:9");
    }
}
