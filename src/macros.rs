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

/// Private macro
///
/// Try to unwrap a `Result`, returns value in `Ok` variant
///
/// If result is `Err`, then run code block
/// Similar to a `let else` statement, but captures the value inside the `Err` variant
macro_rules! try_unwrap {
    (
        $option: expr,
        else Err($err: ident) => $block: block
        $(,)?
    ) => {
        match $option {
            Ok(value) => value,
            Err($err) => $block,
        }
    };

    (
        $option: expr,
        else Err($err: ident) => $stmt: stmt
        $(,)?
    ) => {
        match $option {
            Ok(value) => value,
            // Brackets cannot be removed
            #[rustfmt::skip] Err($err) => { $stmt },
        }
    };
}

/// Private macro
///
/// Shorthand for `Err(crate::Error...)`
macro_rules! fail {
    ( $kind: ident ) => {
        Err($crate::Error::$kind)
    };
    ( $kind: ident, $( $arg: expr ),* ) => {
        Err($crate::Error::$kind( $( $arg ),* ))
    };
}

/// Private macro
///
/// Shorthand for `Err(crate::Error::IoFail(crate::IoError...))`
macro_rules! io_fail {
    ( $kind: ident ) => {
        Err($crate::Error::IoFail($crate::IoError::$kind))
    };
    ( $kind: ident, $( $arg: expr ),* ) => {
        Err($crate::Error::IoFail($crate::IoError::$kind( $( $arg ),* )))
    };
}

#[cfg(test)]
mod tests {
    use crate::{Object, Value};

    #[test]
    fn object_macro_works() {
        let my_key = "hello!";

        let mut obj = Object::new();
        obj.insert("abc".to_string(), Value::from(123));
        obj.insert("array".to_string(), Value::from(vec![4, 5, 6]));
        obj.insert("my_key".to_string(), Value::from(my_key));

        assert_eq!(
            object! {
                abc: 123,
                array: Value::from(vec![4, 5, 6]),
                my_key,
            },
            obj
        );

        let mut obj = Object::new();
        obj.insert("abc".to_string(), Value::from("abcdef"));

        let mut obj2 = Object::new();
        obj2.insert("ghi".to_string(), Value::from(456));

        obj.insert("def".to_string(), Value::Object(obj2));

        assert_eq!(
            object! {
                abc: Value::from("abcdef"),
                def: Value::from(object!{
                    ghi: 456,
                }),
            },
            obj
        );
    }

    #[test]
    fn try_unwrap_works() {
        let result: Result<i32, &str> = Ok(123);
        let value = try_unwrap!( result,
            else Err(_err) => {
                panic!("Should not be Err")
            }
        );
        assert_eq!(value, 123);

        let result: Result<i32, &str> = Err("oh no!");
        let value = try_unwrap!( result,
            else Err(_err) => {
                456
            }
        );
        assert_eq!(value, 456);

        let result: Result<i32, &str> = Err("oh no!");
        let _value = try_unwrap!( result,
            else Err(_err) => {
                return;
            }
        );
        panic!("Should not have been Ok");
    }
}
