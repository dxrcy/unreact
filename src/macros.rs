//TODO Remove!
#[macro_export]
macro_rules! throw {
    ( $lit: literal $(, $arg: expr )* ) => {
        return Err(format!($lit, $( $arg ),*))
    };
}


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
