use std::{
    any::type_name,
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

struct UnexpectedValue<T>(u32, PhantomData<T>);

impl<T> Display for UnexpectedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unexpected value for type {:?}: {}",
            type_name::<T>(),
            self.0
        )
    }
}

impl<T> Debug for UnexpectedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnexpectedValue<{}>({})", type_name::<T>(), self.0)
    }
}

impl<T> Error for UnexpectedValue<T> {}
