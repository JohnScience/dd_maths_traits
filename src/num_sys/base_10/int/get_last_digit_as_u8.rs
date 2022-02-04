// Implementation of the trait via generic impl block doesn't play nicely with
// negative impls because currently (Rust 1.57.0) there is no way to provide guarantee
// that T doesn't implement ArbitrarySizeIntSubset. At the time of writing, negative trait
// bounds are unavailable.
macro_rules! impl_get_last_digit_as_u8 {
    () => {
        fn get_last_digit_as_u8(&self) -> u8 {
            use crate::num_sys::base_10::GetLastDigitBase10AsU8;
            
            self.get_last_digit_base_10_as_u8()
        }
    };
}

#[cfg(any(doc, test, doctest, all(feature = "const_trait_impl", feature = "const_ops")))]
macro_rules! impl_trait_for_t {
    ($trait:path, $t:ty, $fn_macro_name:ident) => {
        impl const $trait for $t {
            $fn_macro_name!();
        }
    };
}

#[cfg(not(any(doc, test, doctest, all(feature = "const_trait_impl", feature = "const_ops"))))]
macro_rules! impl_trait_for_t {
    ($trait:path, $t:ty, $fn_macro_name:ident) => {
        impl $trait for $t {
            $fn_macro_name!();
        }
    };
}

macro_rules! impl_trait {
    ($trait:path, @PRIM_INTS, $fn_macro_name:ident) => {
        impl_trait!($trait, [u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize], $fn_macro_name);
    };
    ($trait:path, [$($t:ty),+], $fn_macro_name:ident) => {
        $(
            impl_trait_for_t!($trait, $t, $fn_macro_name);
        )+
    };
}

impl_trait!(crate::num_sys::GetLastDigitAsU8, @PRIM_INTS, impl_get_last_digit_as_u8);

// TODO: decompose the function and maybe share the results with the library authors
// TODO: track https://github.com/rust-num/num-bigint/issues/233
#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
impl crate::num_sys::GetLastDigitAsU8 for num_bigint::BigUint {
    #[inline]
    fn get_last_digit_as_u8(&self) -> u8 {
        use crate::num_sys::base_10::GetLastDigitBase10AsU8;
        
        self.get_last_digit_base_10_as_u8()
    }
}

#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
impl crate::num_sys::GetLastDigitAsU8 for num_bigint::BigInt {
    #[inline]
    fn get_last_digit_as_u8(&self) -> u8 {
        use crate::num_sys::base_10::GetLastDigitBase10AsU8;

        self.get_last_digit_base_10_as_u8()
    }
}

#[cfg(test)]
mod tests {
    use crate::num_sys::GetLastDigitAsU8;

    #[test]
    fn get_last_digit_as_u8_works_for_negative_primitive_signed_integers() {
        assert_eq!((-17).get_last_digit_as_u8(), 7);
    }

    #[test]
    fn get_last_digit_as_u8_works_for_big_uint_test1() {
        use num_bigint::BigUint;

        let big_uint = BigUint::from(340_282_366_920_938_463_463_374_607_431_768_211_451u128);
        assert_eq!(big_uint.get_last_digit_as_u8(), 1);
    }

    #[test]
    fn get_last_digit_as_u8_works_for_big_uint_test2() {
        use num_bigint::BigUint;

        let big_uint = BigUint::from(340_282_366_920_938_463_463_374_607_431_768_211_452u128);
        assert_eq!(big_uint.get_last_digit_as_u8(), 2);
    }

    #[test]
    fn get_last_digit_as_u8_works_for_big_int_test1() {
        use num_bigint::BigInt;

        let big_int = BigInt::from(-170_141_183_460_469_231_731_687_303_715_884_105_721i128);
        assert_eq!(big_int.get_last_digit_as_u8(), 1);
    }

    #[test]
    fn get_last_digit_as_u8_works_for_big_int_test2() {
        use num_bigint::BigInt;

        let big_int = BigInt::from(-170_141_183_460_469_231_731_687_303_715_884_105_722i128);
        assert_eq!(big_int.get_last_digit_as_u8(), 2);
    }
}
