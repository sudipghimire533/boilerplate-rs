pub trait ResultFlatten {
    type Ok;
    type Err;

    fn flatted(self) -> Result<Self::Ok, Self::Err>;
}

pub trait OptionFlatten {
    type Value;

    fn flatted(self) -> Option<Self::Value>;
}

impl<Value> OptionFlatten for Option<Option<Value>> {
    type Value = Value;

    /// Convert Option<Option<Value>> to Option<Value>
    fn flatted(self) -> Option<Value> {
        self?
    }
}

impl<Ok, Err> ResultFlatten for Result<Result<Ok, Err>, Err> {
    type Ok = Ok;
    type Err = Err;

    /// Convert Result<Result<Value, Err>, Err> to Result<Value, Err>
    fn flatted(self) -> Result<Ok, Err> {
        self?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_flatten() {
        // flatted the inner option
        assert_eq!(Some(Some(1)).flatted(), Some(1));

        // flatten the None vaue is None
        assert_eq!(None::<Option<i32>>.flatted(), None::<i32>);

        // Flattes only once at a time
        assert_eq!(Some(Some(Some(10))).flatted(), Some(Some(10)));
    }

    #[test]
    fn result_flatten() {
        // flatted the inner result
        assert_eq!(Ok(Ok::<i32, ()>(1)).flatted(), Ok(1));

        // flatten the Err value is error
        assert_eq!(Ok(Err::<(), i32>(1)).flatted(), Err(1));

        // flatten only once at a time
        assert_eq!(Ok(Ok::<_, ()>(Ok::<i32, ()>(10))).flatted(), Ok(Ok(10)));
    }
}
