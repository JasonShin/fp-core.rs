use std::collections::HashMap;

// TODO: use a declarative macro (see https://github.com/rust-lang/rust/issues/39412) to make this
// one macro that is invoked repeatedly.

pub trait HKT<U> {
    type Current;
    type Target;
}

macro_rules! derive_hkt {
    ($t: ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type Current = T;
            type Target = $t<U>;
        }
    };
}

derive_hkt!(Option);
derive_hkt!(Vec);

impl<T, U, E> HKT<U> for Result<T, E> {
    type Current = T;
    type Target = Result<U, E>;
}

pub trait HKT3<U1, U2> {
    type Current1;
    type Current2;
    type Target;
}

macro_rules! derive_hkt3 {
    ($t:ident) => {
        impl<T1, T2, U1, U2> HKT3<U1, U2> for $t<T1, T2> {
            // The currently contained types
            type Current1 = T1;
            type Current2 = T2;
            // How the U's get filled in.
            type Target = $t<U1, U2>;
        }
    };
}

derive_hkt3!(HashMap);
