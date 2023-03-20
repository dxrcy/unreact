#[macro_export]
macro_rules! object {
    {} => { $crate::Object::new() };

    {
        $( $key: ident : $value: expr ),* $(,)?
    } => {{
        let mut hm = $crate::Object::new();
        $(
            hm.insert(stringify!($key).to_string(), $crate::Value::from($value));
        )*
        hm
    }};
}

//TODO Remove!
macro_rules! throw {
    ( $lit: literal $(, $arg: expr )* ) => {
        return Err($crate::Error::Generic(format!($lit, $( $arg ),*)))
    };
}

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

#[cfg(test)]
mod tests {
    use crate::{Object, Value};

    #[test]
    fn object_macro_works() {
        let mut obj = Object::new();
        obj.insert("abc".to_string(), Value::from(123));

        assert_eq!(
            object! {
                abc: Value::from(123)
            },
            obj
        );

        let mut obj = Object::new();
        obj.insert("abc".to_string(), Value::from(123));

        let mut obj2 = Object::new();
        obj2.insert("ghi".to_string(), Value::from(456));

        obj.insert("def".to_string(), Value::Object(obj2));

        assert_eq!(
            object! {
                abc: Value::from(123),
                def: Value::from(object!{
                    ghi: Value::from(456),
                }),
            },
            obj
        );
    }
}
