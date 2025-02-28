use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Stack {
    pub inner: Vec<String>,
}

impl Stack {
    pub fn new(inner: Vec<String>) -> Self {
        Stack { inner }
    }
}

#[derive(Debug)]
pub struct Thing {
    pub msg: String,
    pub inner: Vec<String>,
}

impl Error for Thing {}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

// impl Value for Stack {
//     /// Visits this value with the given `Visitor`.
//     fn record(&self, key: &Field, visitor: &mut dyn Visit) {
//         visitor.record_debug(key, &self.inner);
//     }
// }

/// A macro called e.g. as `err_struct(A, B => C)` will define a public struct by name `C` and implement `From<A> for C` and `From<B> for C`.
/// The from types are optional, and `err_struct(A, B => C)` equates to `err_struct(A, B => C)`, `err_from(A, C)` and `err_from(B, C)`.
#[macro_export]
macro_rules! err_struct {
    ($target: ident) => {
        #[derive(Debug)]
        pub struct $target {
            pub msg: String,
            pub stack: error_land::Stack,
        }

        impl $target {
            pub fn to_error(&self) -> Box<dyn std::error::Error + 'static> {
                Box::new(error_land::Thing { msg: self.msg.clone(), inner: self.stack.inner.clone() })
            }
        }

        impl std::fmt::Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //write!(f, "FOOBAR {} {} {:?}", stringify!($target), self.msg, self.stack);
                write!(f, "{} {}", stringify!($target), self.msg)
                // for item in self.stack.inner.clone() {
                //     write!(f, "Display {item}");
                // }
                //Ok(())
            }
        }

        // impl std::fmt::Debug for $target {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         write!(f, "{{\"msg\": \"{}\"}}", self.msg)
        //     }
        // }

        // Note that $target must not implement std::error::Error because
        // From trait has blanket implementation. of blanket impl<T> From<T> for T

        impl<E: std::error::Error> From<E> for $target {
            #[track_caller]
            #[inline(always)]
            fn from(e: E) -> Self {
                let caller = std::panic::Location::caller().to_string();
                //tracing::error!(caller=caller, message=e.to_string());
                //Self { msg: format!("{}", e), stack: error_land::Stack::new(vec![format!("{} {}", e, caller)]) }
                Self { msg: format!("{}", e), stack: error_land::Stack::new(vec![caller]) }
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
                let msg = error.to_string();
                let mut stack = error.stack;
                //stack.inner.push(format!("{} {}", msg.clone(), caller));
                stack.inner.push(caller);
                Self {msg, stack}
            }
        }
        )+
    };
}
