// The declaration below relies on https://github.com/rust-lang/rfcs/pull/1657
#[cfg(all(
    feature = "gen_associated_consts",
    any(doc, test, doctest, feature = "adt_const_params")
))]
pub trait SumDigits {
    fn sum_digits<
        Base,
        Accumulator,
        Modulo,
        Output,
        const BASE: Base,
        const MODULO: Option<Modulo> = None,
    >(
        &self,
    ) -> Output
    where
        Self: Sized,
        (Self, Base, Accumulator, Modulo, Output): SumDigitsTypeTuple;
}

type DefaultBase = u128;
type DefaultModulo = u128;

#[cfg(all(
    not(feature = "gen_associated_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
pub trait SumDigits {
    fn sum_digits<Base, Accumulator, Modulo, Output, const BASE: DefaultBase, const MODULO: Option<DefaultModulo>>(
        &self,
    ) -> Output
    where
        Self: Sized,
        (Self, Base, Accumulator, Modulo, Output): SumDigitsTypeTuple;
}

pub trait SumDigitsTypeTuple {}

pub trait SumDigitsDefaults {
    type Base;
    type Accumulator;
    type Modulo;
    type Output;
}

impl<T, Base, Accumulator, Modulo, Output> SumDigitsTypeTuple
    for (T, Base, Accumulator, Modulo, Output)
where
    T: SumDigitsDefaults,
{
}

// TODO: consider a need for distinguishing implementations based on whether the
// modulo operation is even needed, whether the number of modulo operations is known, an
// if it is known, what is the count of modulo operations
impl SumDigitsDefaults for u8 {
    type Base = DefaultBase;
    type Accumulator = u8;
    type Modulo = u8;
    // 199 is the number with the highest sum of digits in Accumulator.
    // Output must be able to hold values from 0 to 1+9+9=19
    type Output = u8;
}