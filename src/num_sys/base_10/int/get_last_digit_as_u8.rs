// Implementation of the trait via generic impl block doesn't play nicely with
// negative impls because currently (Rust 1.57.0) there is no way to provide guarantee
// that T doesn't implement ArbitrarySizeIntSubset. At the time of writing, negative trait
// bounds are unavailable.
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
macro impl_get_last_digit_as_u8_for_prim_signed_int($t:ty) {
    impl crate::num_sys::GetLastDigitAsU8 for $t {
        fn get_last_digit_as_u8(&self) -> u8 {
            const DEFAULT_RADIX: $t = 10;
            let rem: $t = self % DEFAULT_RADIX;
            // The line below is necessary only for signed integers
            let last_digit: $t = if rem < 0 { -rem } else { rem };
            // unwrap is safe because last_digit < 10
            last_digit.try_into().unwrap()
        }
    }
}

#[cfg(not(any(doc, test, doctest, feature = "decl_macro")))]
macro_rules! impl_get_last_digit_as_u8_for_prim_signed_int {
    ($t:ty) => {
        impl crate::num_sys::GetLastDigitAsU8 for $t {
            fn get_last_digit_as_u8(&self) -> u8 {
                const DEFAULT_RADIX: $t = 10;
                let rem: $t = self % DEFAULT_RADIX;
                // The line below is necessary only for signed integers
                let last_digit: $t = if rem < 0 { -rem } else { rem };
                // unwrap is safe because last_digit < 10
                last_digit.try_into().unwrap()
            }
        }
    };
}

// Implementation of the trait via generic impl block doesn't play nicely with
// negative impls because currently (Rust 1.57.0) there is no way to provide guarantee
// that T doesn't implement ArbitrarySizeIntSubset. At the time of writing, negative trait
// bounds are unavailable.
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
macro impl_get_last_digit_as_u8_for_prim_unsigned_int($t:ty) {
    impl crate::num_sys::GetLastDigitAsU8 for $t {
        fn get_last_digit_as_u8(&self) -> u8 {
            const DEFAULT_RADIX: $t = 10;
            let rem: $t = self % DEFAULT_RADIX;
            // unwrap is safe because last_digit < 10
            rem.try_into().unwrap()
        }
    }
}

#[cfg(not(any(doc, test, doctest, feature = "decl_macro")))]
macro_rules! impl_get_last_digit_as_u8_for_prim_unsigned_int {
    ($t:ty) => {
        impl crate::num_sys::GetLastDigitAsU8 for $t {
            fn get_last_digit_as_u8(&self) -> u8 {
                const DEFAULT_RADIX: $t = 10;
                let rem: $t = self % DEFAULT_RADIX;
                // unwrap is safe because last_digit < 10
                rem.try_into().unwrap()
            }
        }
    };
}

impl_get_last_digit_as_u8_for_prim_signed_int!(i8);
impl_get_last_digit_as_u8_for_prim_signed_int!(i16);
impl_get_last_digit_as_u8_for_prim_signed_int!(i32);
impl_get_last_digit_as_u8_for_prim_signed_int!(i64);
impl_get_last_digit_as_u8_for_prim_signed_int!(i128);
impl_get_last_digit_as_u8_for_prim_signed_int!(isize);

impl_get_last_digit_as_u8_for_prim_unsigned_int!(u8);
impl_get_last_digit_as_u8_for_prim_unsigned_int!(u16);
impl_get_last_digit_as_u8_for_prim_unsigned_int!(u32);
impl_get_last_digit_as_u8_for_prim_unsigned_int!(u64);
impl_get_last_digit_as_u8_for_prim_unsigned_int!(u128);
impl_get_last_digit_as_u8_for_prim_unsigned_int!(usize);

// TODO: decompose the function and maybe share the results with the library authors
// TODO: track https://github.com/rust-num/num-bigint/issues/233
#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
impl crate::num_sys::GetLastDigitAsU8 for num_bigint::BigUint {
    fn get_last_digit_as_u8(&self) -> u8 {
        const DEFAULT_RADIX: u8 = 10;
        // Unfortunately, the library doesn't offer a way to return the remainder
        // of the divisor's type even for primitive integers.
        let rem: Self = self % DEFAULT_RADIX;
        let rem: u32 = {
            let least_significant_digit: u32 = {
                // TODO: contact the library authors to make endianness explicit. Read more about endianness:
                // https://en.wikipedia.org/wiki/Endianness
                let mut le_iter_u32 = rem.iter_u32_digits();
                le_iter_u32
                    .next()
                    // integers always have at least one digit
                    .unwrap()
            };
            least_significant_digit
        };
        let rem: u8 = rem
            .try_into()
            // that number is the remainder of divison by 10 and therefore < 10 < 256
            .unwrap();
        rem
    }
}

#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
impl crate::num_sys::GetLastDigitAsU8 for num_bigint::BigInt {
    fn get_last_digit_as_u8(&self) -> u8 {
        self.magnitude().get_last_digit_as_u8()
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
