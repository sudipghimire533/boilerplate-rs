/// Get something
pub trait Get<T> {
    /// Get something
    fn get() -> T;
}

/// Get the lngth of sttaic array only from type
pub trait LengthOfArray {
    const LENGTH: usize;
}
impl<T, const LENGTH: usize> LengthOfArray for [T; LENGTH] {
    const LENGTH: usize = LENGTH;
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncGet<T> {
    /// Get something asynchronously
    async fn get() -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_type() {
        struct MagicNumberGetter;

        impl Get<i32> for MagicNumberGetter {
            fn get() -> i32 {
                42
            }
        }

        assert_eq!(MagicNumberGetter::get(), 42);
    }

    #[test]
    fn array_length() {
        type Public = [u8; 32];

        assert_eq!(Public::LENGTH, 32);
    }
}
