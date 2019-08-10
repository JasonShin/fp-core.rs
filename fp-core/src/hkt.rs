pub trait HKT<U> {
    type Current; // The current contained type
    type Target; // The next type (how to generically fill in U)
}

macro_rules! derive_hkt {
    ($t:ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type Current = T;
            type Target = $t<U>;
        }
    };
}

derive_hkt!(Option);
derive_hkt!(Vec);

// TODO: what is this?
pub trait HKT3<A, B, C> {
    type Target2;
}

impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}
