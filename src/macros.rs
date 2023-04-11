/// Create a json-like 'object':
/// A map of string keys to json values
///
/// `unreact::Object` is a type alias for `serde_json::Map<String, serde_json::Value>`
///
/// Similar to `serde_json::json!` macro, but must be an object
///
/// # Examples
///
/// ```
/// # use unreact::object;
/// let my_key = "Hello!";
/// object! {
///     foo: 123,
///     bar: vec![4, 5, 6],
///     // Use variable with same name as key
///     my_key,
///     // Nested objects must also use `object!` macro
///     nested: object! {
///         key: "value"
///     }
/// };
/// ```
///
/// The above code is equivalent to this json:
///
/// ```json
/// {
///     "foo": 123,
///     "bar": [4, 5, 6],
///     "my_key": "Hello!",
///     "nested": {
///         "key": "value"
///     }
/// }
/// ```
#[macro_export]
macro_rules! object {
    // Empty object
    {} => { $crate::Object::new() };

    // Object
    {
        $( $key: ident $(: $value: expr)? ),* $(,)?
    } => {{
        let mut hm = $crate::Object::new();
        $(
            object!(@entry hm, $key $(: $value)?);
        )*
        hm
    }};

    // Key, no value
    (@entry $hm: expr,
        $key: ident
    ) => {
        $hm.insert(String::from(stringify!($key)), $crate::json!($key));
    };

    // Key and value
    (@entry $hm: expr,
        $key: ident : $value: expr
    ) => {
        $hm.insert(String::from(stringify!($key)), $crate::json!($value));
    };
}

/// **development only**
///
/// Return a generic error type, with a formatted string
macro_rules! throw {
    (
        $literal: literal $(, $arg: expr )* $(,)?
    ) => {
        return Err($crate::Error::Generic(
            format!($literal $(, $arg )*)
        ))
    };
}

/// **development only**
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
