pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

macro_rules! semigroup_numeric_impl {
    ($($t:ty)*) => ($(
        impl Semigroup for $t {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
    )*)
}

semigroup_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl<T: Clone> Semigroup for Vec<T> {
    fn combine(self, other: Self) -> Self {
        let mut concat = self.to_vec();
        concat.extend_from_slice(&other);
        concat
    }
}

impl Semigroup for String {
    fn combine(self, other: Self) -> Self {
        format!("{}{}", self, other)
    }
}
