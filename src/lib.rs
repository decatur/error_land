mod loc_error;
mod loc_formater;
mod loc_layer;

pub use loc_error::{Stack, Thing};
pub use loc_formater::{JsonFormatter, PrettyFormatter};
pub use loc_layer::{JsonLayer, PrettyLayer};

/*
/// A macro called e.g. as `err_struct(A, B => C)` will define a public struct by name `C` and implement `From<A> for C` and `From<B> for C`.
/// The from types are optional, and `err_struct(A, B => C)` equates to `err_struct(A, B => C)`, `err_from(A, C)` and `err_from(B, C)`.
#[macro_export]
macro_rules! err_struct {
    ($target: ident) => {
        #[derive(Debug)]
        pub struct $target(String);

        impl $target {
            #[track_caller]
            #[inline(always)]
            pub fn new(msg: String) -> Self {
                let caller = std::panic::Location::caller().to_string();
                error!(caller=caller, message=msg);
                $target(msg)
            }
        }

        impl std::fmt::Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", stringify!($target), self.0)
            }
        }

        // Note that $target must not implement std::error::Error because
        // From trait has blanket implementation. of blanket impl<T> From<T> for T

        impl<E: std::error::Error> From<E> for $target {
            #[track_caller]
            #[inline(always)]
            fn from(e: E) -> Self {
                // let caller = std::panic::Location::caller().to_string();
                // error!(caller=caller, message=e.to_string());
                Self::new(e.to_string())
            }
        }
    };
    ($($source:ident),+ => $target:ident) => {
        err_struct!($target);
        $(
            err_from!($source, $target);
        )+

      };
}

/// A call to `err_from(B, C)` implements `From<B> for C` AND outputs the location of the caller.
#[macro_export]
macro_rules! err_from {
    ($source: ident, $target: ident) => {
        impl From<$source> for $target {
            #[track_caller]
            fn from(error: $source) -> Self {
                let caller = std::panic::Location::caller().to_string();
                error!(caller=caller
                //    , message=error.to_string()
                );
                Self(error.to_string())
            }
        }
    };
}*/
