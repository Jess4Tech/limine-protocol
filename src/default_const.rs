/// A trait for const defaults
pub trait ConstDefault {
    /// The default value
    const DEFAULT: Self;
}

impl<T> ConstDefault for Option<T> {
    const DEFAULT: Self = None;
}

impl<T> ConstDefault for *const T {
    const DEFAULT: Self = core::ptr::null();
}

impl<T> ConstDefault for *mut T {
    const DEFAULT: Self = core::ptr::null_mut();
}

/// Implement the default values for numbers
macro_rules! impl_number_constants {
    ($($ty:ty$(,)*)*) => {
        $(
        impl ConstDefault for $ty {
            const DEFAULT: Self = Self::MIN;
        }
        )*
    };
}

impl_number_constants!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
