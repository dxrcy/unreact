/// Panic with a custom message if unwrap fails
///
/// For `server` module only!
macro_rules! unwrap {
    (
        $result: expr,
        $($err: ident : )? $msg: literal $(, $( $arg: expr ),*)?
    ) => {
        $result.unwrap_or_else(|#[allow(unused_variables)] err|
            panic!("[dev] Failed! {}",
                format!($msg, $( $( $arg, )*)? $( $err = err )?),
            )
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_works() {
        let result: Result<i32, &str> = Ok(123);
        assert_eq!(unwrap!(result, "uh oh!"), 123);
    }

    #[test]
    #[should_panic]
    fn unwrap_panics() {
        let result: Result<i32, &str> = Err("whoops!");
        unwrap!(result, "uh oh!");
    }

    #[test]
    #[should_panic]
    fn unwrap_panics_with_message() {
        let result: Result<i32, &str> = Err("whoops!");
        unwrap!(result, "uh oh! {}", 1);
        unwrap!(result, err: "uh oh! {} `{err:?}`", 1);
        unwrap!(result, err: "uh oh! `{err:?}`");
    }
}
