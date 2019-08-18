use std::char::from_u32;

pub trait Empty {
    fn empty() -> Self;
}

macro_rules! impl_empty_i {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                0
            }
        }
    )*)
}

impl_empty_i! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! impl_empty_f {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                0.0
            }
        }
    )*)
}

impl_empty_f! { f32 f64 }

impl Empty for String {
    fn empty() -> Self {
        "".into()
    }
}
