/// (development only)
///
/// Return a generic error type, with a formatted string
macro_rules! throw {
    (
        $literal: literal $(, $arg: expr )* $(,)?
    ) => {
        return Err(Error::Generic(
            format!($literal $(, $arg )*)
        ))
    };
}

/// (development only)
///
/// Try to unwrap a `Result`, or run expression
///
/// Wrapper for `match` statement
macro_rules! try_else {
    // With error argument
    (
        try $expr: expr,
        else $err: ident :
        $($tt:tt)*
    ) => {
        match $expr {
            Ok(x) => x,
            Err($err) => $($tt)*
        }
    };

    // No error argument
    (
        try $expr: expr,
        else
        $($tt:tt)*
    ) => {
        match $expr {
            Ok(x) => x,
            Err(_) => $($tt)*
        }
    };
}
