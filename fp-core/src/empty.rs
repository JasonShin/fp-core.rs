use std::char::from_u32;

pub trait Empty {
    fn empty() -> Self;
}

macro_rules! impl_empty_nums {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                Self::empty()
            }
        }
    )*)
}

impl_empty_nums! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl Empty for String {
    fn empty() -> Self {
        "".into()
    }
}
