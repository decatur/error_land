pub mod error_string;
mod json_formater;
mod loc_error;

use core::fmt;

pub use json_formater::{as_display, as_value, JsonFormatter};
pub use loc_error::{CoreError, StackItem};

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
    use crate::{err_from, err_struct, into_err};

    err_struct!(ErrorB => ErrorA);
    fn a() -> Result<(), ErrorA> {
        b()?;
        Ok(())
    }

    err_struct!(ErrorB);
    fn b() -> Result<(), ErrorB> {
        Err(into_err("fn b() did bad"))?
    }

    #[test]
    fn test() {
        let err = a().err().unwrap();
        println!("{}", err);
        assert_eq!(
            err.to_string(),
            r#"{"backtrace":[{"source":"Error(\"fn b() did bad\")","target":"ErrorB","msg":"fn b() did bad","location":"src/lib.rs:43:9"},{"source":"ErrorB","target":"ErrorA","msg":"","location":"src/lib.rs:37:9"}]}"#
        );
    }
}
