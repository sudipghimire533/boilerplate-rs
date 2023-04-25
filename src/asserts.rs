#[macro_export]
macro_rules! assert_ok {
    ($cond:expr,) => {
        $crate::assert_ok!($cond);
    };

    ($cond:expr) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?})", e);
            }
        }
    };

    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?}): {}", e, format_args!($($arg)+));
            }
        }
    };
}

#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok!($($arg)*); })
}

#[macro_export]
macro_rules! assert_err {
    ($cond:expr,) => {
        $crate::assert_err!($cond);
    };
    ($cond:expr) => {
        match $cond {
            Ok(t) => {
                panic!("assertion failed, expected Err(..), got Ok({:?})", t);
            },
            Err(e) => e,
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => {
                panic!("assertion failed, expected Err(..), got Ok({:?}): {}", t, format_args!($($arg)+));
            },
            Err(e) => e,
        }
    };
}

#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err!($($arg)*); })
}

#[macro_export]
macro_rules! assert_ok_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_ok_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Ok(t) => {
                assert_eq!(t, $expected);
                t
            },
            e @ Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}", e);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => {
                assert_eq!(t, $expected);
                t
            },
            e @ Err(..) => {
                panic!("assertion failed, expected Ok(..), got {:?}: {}", e, format_args!($($arg)+));
            }
        }
    };
}

#[macro_export]
macro_rules! debug_assert_ok_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok_eq!($($arg)*); })
}
