/// Define a single type wrapper only type
/// Generate a type alias for inner type and implement
/// trait to access inner type
#[macro_export]
macro_rules! define_wrapper_type {
    ($name: ident, $in_type: ty) => {
        #[repr(transparent)]
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub struct $name($in_type);

        ::paste::paste! {
            #[allow(non_camel_case_types)]
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
            pub fn consume(self) -> $in_type {
                self.0
            }
        }
    };
}
