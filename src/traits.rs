/// Get something
pub trait Get<T> {
    /// Get something
    fn get(&self) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_type() {
        struct I32Getter(i32);

        impl Get<i32> for I32Getter {
            fn get(&self) -> i32 {
                self.0
            }
        }

        impl Get<i64> for I32Getter {
            fn get(&self) -> i64 {
                self.0.into()
            }
        }

        let getter = I32Getter(42);
        let num_i32: i32 = getter.get();
        let num_i64: i64 = getter.get();

        assert_eq!(num_i64, num_i32.into());
    }
}
