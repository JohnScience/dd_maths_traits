// The declaration below relies on https://github.com/rust-lang/rfcs/pull/1657
#[cfg(all(
    feature = "gen_assoc_consts",
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
        (Self, Base, Accumulator, Modulo, Output): SumDigitsTypeTuple<
            Number = Self,
            Base = Base,
            Accumulator = Accumulator,
            OptModulo = Modulo,
            Output = Output,
        >,
    {
        <(Self, Base, Accumulator, Modulo, Output) as SumDigitsTypeTuple>::sum_digits::<BASE, MODULO>(
            self,
        )
    }
}

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
type DefaultBase = u128;
#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
type DefaultOptModulo = Option<u128>;

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
pub trait SumDigits {
    fn sum_digits<
        Base,
        Accumulator,
        OptModulo,
        Output,
        const BASE: DefaultBase,
        const OPT_MODULO: DefaultOptModulo,
    >(
        &self,
    ) -> Output
    where
        Self: Sized,
        (Self, Base, Accumulator, OptModulo, Output): SumDigitsTypeTuple<
            Number = Self,
            Base = Base,
            Accumulator = Accumulator,
            OptModulo = OptModulo,
            Output = Output,
        >,
    {
        <(Self, Base, Accumulator, OptModulo, Output) as SumDigitsTypeTuple>::sum_digits::<
            BASE,
            OPT_MODULO,
        >(self)
    }
}

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
impl<T> SumDigits for T where T: SumDigitsDefaults {}

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
pub trait SumDigitsTypeTuple {
    type Number;
    type Base;
    type Accumulator;
    type OptModulo;
    type Output;
    fn sum_digits<const BASE: DefaultBase, const OPT_MODULO: DefaultOptModulo>(
        n_ref: &Self::Number,
    ) -> Self::Output;
}

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
impl<Number, Base, Accumulator, OptModulo, Output> SumDigitsTypeTuple
    for (Number, Base, Accumulator, OptModulo, Output)
where
    Number: SumDigitsDefaults<
        Base = Base,
        Accumulator = Accumulator,
        OptModulo = OptModulo,
        Output = Output,
    >,
{
    type Number = Number;
    type Base = Base;
    type Accumulator = Accumulator;
    type OptModulo = OptModulo;
    type Output = Output;
    fn sum_digits<const BASE: DefaultBase, const OPT_MODULO: DefaultOptModulo>(
        n_ref: &Self::Number,
    ) -> Self::Output {
        n_ref.generic_std_sum_digits::<BASE, OPT_MODULO>()
    }
}

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
pub trait SumDigitsDefaults {
    type Base;
    type Accumulator;
    type OptModulo;
    type Output;
    const BASE: DefaultBase;
    const NO_MODULO: DefaultOptModulo;

    fn std_sum_digits(&self) -> Self::Output;
    fn generic_std_sum_digits<const BASE: DefaultBase, const OPT_MODULO: DefaultOptModulo>(
        &self,
    ) -> Self::Output;
}

#[cfg(all(
    not(feature = "gen_assoc_consts"),
    any(doc, test, doctest, feature = "adt_const_params")
))]
// TODO: consider a need for distinguishing implementations based on whether the
// modulo operation is even needed, whether the number of modulo operations is known, and
// if it is known, what the count of modulo operations is
impl SumDigitsDefaults for u8 {
    type Base = u8;
    type Accumulator = u8;
    type OptModulo = u8;
    // 199 is the number with the highest sum of digits in Accumulator.
    // Output must be able to hold values from 0 to 1+9+9=19
    type Output = u8;

    const BASE: DefaultBase = 10;
    const NO_MODULO: DefaultOptModulo = None;

    fn generic_std_sum_digits<const BASE: DefaultBase, const OPT_MODULO: DefaultOptModulo>(
        &self,
    ) -> Self::Output {
        let base: u8 = if BASE > 255 { 255 } else { BASE as u8 };
        let mut acc: Self::Accumulator = 0;
        let mut q = *self;
        loop {
            let (new_q, r) = (q / base, q % base);
            q = new_q;
            acc += r;
            if q == 0 {
                break;
            };
        }
        match OPT_MODULO {
            None => acc,
            Some(modulo) => {
                if modulo >= 255 {
                    acc
                } else {
                    acc % (modulo as u8)
                }
            }
        }
    }

    #[inline(always)]
    fn std_sum_digits(&self) -> Self::Output {
        self.generic_std_sum_digits::<{ Self::BASE }, { Self::NO_MODULO }>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_digits_works_for_default_type_tuple_of_u8() {
        use crate::num_sys::base_10::int::sum_digits::{
            DefaultBase, DefaultOptModulo, SumDigitsTypeTuple,
        };

        const BASE_10: DefaultBase = 10;
        const NO_MODULO: DefaultOptModulo = None;
        assert_eq!(
            <(u8, u8, u8, u8, u8) as SumDigitsTypeTuple>::sum_digits::<BASE_10, NO_MODULO>(&5u8),
            5u8
        );
    }

    #[test]
    fn sum_digits_for_5u8() {
        use crate::num_sys::base_10::int::sum_digits::SumDigitsDefaults as D;

        assert_eq!(5u8.std_sum_digits(), 5u8)
    }

    #[test]
    fn sum_digits_mod_2_works_for_5u8() {
        use crate::num_sys::base_10::int::sum_digits::{
            DefaultBase, DefaultOptModulo, SumDigits, SumDigitsDefaults as D,
        };

        // https://internals.rust-lang.org/t/named-type-parameters/6921/20
        type Base = <u8 as D>::Base;
        type OptModulo = <u8 as D>::OptModulo;
        type Accumulator = <u8 as D>::Accumulator;
        type Output = <u8 as D>::Output;
        const BASE_10: DefaultBase = <u8 as D>::BASE;
        const MODULO_2: DefaultOptModulo = Some(2);
        assert_eq!(
            5u8.sum_digits::<Base, OptModulo, Accumulator, Output, BASE_10, MODULO_2>(),
            1u8
        )
    }
}
