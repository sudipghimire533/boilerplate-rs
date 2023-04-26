/// Define a single type wrapper only type
/// Generate a type alias for inner type and implement
/// trait to access inner type
#[macro_export]
macro_rules! define_wrapper_type {
    // macro handle to receive derive traits defination
    ($name: ident, $in_type: ty, $(#[$m: meta]),* ; $(#[$n: meta]),*) => {
        $(#[$m])*
        pub struct $name( $(#[$n])* $in_type );

        ::paste::paste! {
            #[allow(non_camel_case_types, dead_code)]
            #[doc = "Inner type of `" $name "`"]
            pub type [<$name _InnerType>] = $in_type;
        }

        impl From<$in_type> for $name {
            fn from(inner: $in_type) -> Self {
                $name(inner)
            }
        }

        impl AsRef<$in_type> for $name {
            fn as_ref(&self) -> &$in_type {
                &self.0
            }
        }

        impl AsMut<$in_type> for $name {
            fn as_mut(&mut self) -> &mut $in_type {
                &mut self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            #[doc = "Consume the wrapper and return the inner value as is"]
            pub fn consume(self) -> $in_type {
                self.0
            }
        }

    };

    ($name: ident, $in_type: ty) => {
        define_wrapper_type!(
            $name,
            $in_type,
            #[repr(transparent)],
            #[derive(Debug, Clone, Eq, PartialEq)];
        );
   };
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    #[test]
    fn wrapper_type() {
        define_wrapper_type!(Value, u32);

        let val = Value::from(1);

        // should derive clone trait
        assert_eq!(<Value as Clone>::clone(&val), val);

        // should derive PartialEq
        assert!(<Value as PartialEq>::eq(&val, &val));

        // should implement following Debug trait as well
        let _check_derive: Box<dyn std::fmt::Debug> = Box::new(val.clone());

        // Should derive AsRef
        assert_eq!(<Value as AsRef<u32>>::as_ref(&val), &1);

        // should have a consume to return the inner value as is
        assert_eq!(val.consume(), 1);

        // Type alias $Name + _InnerType should be defined referring to inner type
        assert_eq!(Value::from(10_u32).consume(), Value_InnerType::from(10_u32));
    }

    #[test]
    fn wraper_type_core() {
        define_wrapper_type!(Custom, (i32, &'static str), #[derive(Hash)], #[repr(transparent)];);

        let val = Custom::from((10, "test"));

        let _can_be_hashed = <Custom as Hash>::hash(&val, &mut DefaultHasher::new());
    }
}
