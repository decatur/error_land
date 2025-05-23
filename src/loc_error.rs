use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct StackItem {
    pub msg: String,
    pub location: String,
    pub source: String,
    pub target: String,
}

#[derive(Debug)]
pub struct CoreError {
    pub backtrace: Vec<StackItem>,
}

impl Error for CoreError {}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.backtrace)
    }
}

/// A macro called e.g. as `err_struct(A, B => C)` will define a public struct by name `C` and implement `From<A> for C` and `From<B> for C`.
/// The from types are optional, and `err_struct(A, B => C)` equates to `err_struct(A, B => C)`, `err_from(A, C)` and `err_from(B, C)`.
#[macro_export]
macro_rules! err_struct {
    ($target: ident) => {
        #[derive(Debug)]
        pub struct $target {
            backtrace: Vec<$crate::StackItem>,
        }

        impl $target {
            /// Consuming conversion to a std::error::Error.
            pub fn to_error(self) -> Box<dyn std::error::Error + 'static> {
                Box::new($crate::CoreError { backtrace: self.backtrace })
            }
        }

        impl std::fmt::Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //write!(f, "FOOBAR {} {} {:?}", stringify!($target), self.msg, self.stack);
                write!(f, "{{\"backtrace\":[")?;
                for (index, item) in self.backtrace.clone().iter().enumerate() {
                    if index > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{{\"source\":{},\"target\":{},\"msg\":{},\"location\":{}}}",
                    serde_json::to_string(&item.source).unwrap(),
                    serde_json::to_string(&item.target).unwrap(),
                    serde_json::to_string(&item.msg).unwrap(),
                    serde_json::to_string(&item.location).unwrap())?;
                }
                write!(f, "]}}")
                //Ok(())
            }
        }

        // Note that $target must not implement std::error::Error because
        // From trait has blanket implementation. of blanket impl<T> From<T> for T

        impl<E: std::error::Error> From<E> for $target {
            #[track_caller]
            #[inline(always)]
            fn from(e: E) -> Self {
                let caller = std::panic::Location::caller().to_string();
                let stack_item = $crate::StackItem { msg: format!("{}", e), location: caller, source:format!("{:?}", e), target: stringify!($target).to_owned()};
                Self { backtrace: vec![stack_item] }
            }
        }
    };
    ($($source:ident),+ => $target:ident) => {
        err_struct!($target);
        $(
            err_from!($source => $target);
        )+

      };
}

/// A call to `err_from(B, C)` implements `From<B> for C` AND logs the location of the caller.
#[macro_export]
macro_rules! err_from {
    ($($source:ident),+ => $target: ident) => {
        $(
        impl From<$source> for $target {
            #[track_caller]
            fn from(error: $source) -> Self {
                let caller = std::panic::Location::caller().to_string();
                let mut backtrace = error.backtrace;
                let stack_item = $crate::StackItem{msg: "".to_owned(), location:caller, source: stringify!($source).to_owned(), target: stringify!($target).to_owned(),};
                backtrace.push(stack_item);
                Self {backtrace}
            }
        }
        )+
    };
}
