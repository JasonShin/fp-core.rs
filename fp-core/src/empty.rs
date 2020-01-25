pub trait Empty {
    fn empty() -> Self;
}

macro_rules! numeric_empty_impl {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                0
            }
        }
    )*)
}

numeric_empty_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! floating_numeric_empty_impl {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                0.0
            }
        }
    )*)
}

floating_numeric_empty_impl! { f32 f64 }

impl<T> Empty for Vec<T> {
    fn empty() -> Vec<T> {
        vec![]
    }
}

impl Empty for String {
    fn empty() -> String {
        "".to_string()
    }
}

impl<A> Empty for Option<A> {
    fn empty() -> Option<A> {
        Option::None
    }
}

impl<A, E: Empty> Empty for Result<A, E> {
    fn empty() -> Result<A, E> {
        Result::Err(E::empty())
    }
}
