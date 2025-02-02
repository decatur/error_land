

// #[allow(dead_code)]
// #[derive(Debug)]
// pub struct MyError {
//     pub source: Box<dyn std::fmt::Debug>,
//     pub location: &'static std::panic::Location<'static>,
// }

// #[track_caller]
// pub fn from_error<E: std::fmt::Debug + 'static>(e: E) -> MyError {
//     tracing::error!(
//         "{} {}",
//         std::any::type_name::<E>(),
//         std::panic::Location::caller()
//     );
//     MyError { source: Box::new(e), location: std::panic::Location::caller() }
// }

/// A macro called e.g. as `err_struct(A, B => C)` will define a public struct by name `C` and implement `From<A> for C` and `From<B> for C`.
/// The from types are optional, and `err_struct(A, B => C)` equates to `err_struct(A, B => C)`, `err_from(A, C)` and `err_from(B, C)`.
#[macro_export]
macro_rules! err_struct {
    ($s: ident) => {
        #[derive(Debug)]
        pub struct $s(pub String);

        // Note that $s must not implement std::error::Error because From trait has blanket implementation. of blanket impl<T> From<T> for T

        impl<E: std::error::Error> From<E> for $s {
            fn from(e: E) -> Self {
                Self(e.to_string())
            }
        }
    };
    ($($s:ident),+ => $ss:ident) => {
        err_struct!($ss);
        $(
            err_from!($s, $ss);
        )+
 
      };
}

/// 
#[macro_export]
macro_rules! err_from {
    ($source: ident, $target: ident) => {
        impl From<$source> for $target {
            #[track_caller]
            fn from(error: $source) -> Self {
                let caller = std::panic::Location::caller();
                eprintln!(
                    "{} {} {}",
                    std::any::type_name::<$source>(),
                    //std::any::type_name::<Self>(),
                    caller,
                    error.0
                );
                Self(error.0)
            }
        }
    };
}

// #[macro_export] macro_rules! from {
//     ($source: ident, $target: ident) => {
//         impl From<$source> for $target {
//             #[track_caller]
//             fn from(error: $source) -> Self {
//                 let caller = std::panic::Location::caller();
//                 tracing::error!(
//                     "{} {} {}",
//                     std::any::type_name::<$source>(),
//                     std::any::type_name::<Self>(),
//                     caller,
//                 );
//                 Self(MyError {
//                     location: caller,
//                     source: Box::new(error),
//                 })
//             }
//         }
//     };
// }

// #[macro_export] macro_rules! from_string {
//     ($target: ident) => {
//         impl From<String> for $target {
//             #[track_caller]
//             fn from(error: String) -> Self {
//                 let caller = std::panic::Location::caller();
//                 tracing::error!(
//                     "{} {} {}",
//                     error,
//                     std::any::type_name::<Self>(),
//                     caller
//                 );
//                 Self(MyError {
//                     location: caller,
//                     source: Box::new(error),
//                 })
//             }
//         }
//     };
// }

// fn type_name_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }

// pub fn caller_name() -> String {
//     // See https://docs.rs/stdext/latest/src/stdext/macros.rs.html#63-74
//     // See https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust

//     fn f() {
//     }

//     let name = type_name_of(f);
//     // Examples:
//     //  playground::main::{{closure}}::f
//     //  playground::main::f

//     let right_trim = if name.ends_with("::{{closure}}::f") {
//         16
//     } else {
//         3
//     };

//     name[..name.len() - right_trim].to_owned()
// }
