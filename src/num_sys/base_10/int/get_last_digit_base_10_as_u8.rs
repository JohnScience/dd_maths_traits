macro_rules! impl_get_last_digit_base_10_as_u8_for_prim_signed_int {
    () => {
        fn get_last_digit_base_10_as_u8(&self) -> u8 {
            let rem: Self = self % 10;
            // The line below is necessary only for signed integers
            let last_digit: Self = if rem < 0 { -rem } else { rem };
            // cast doesn't wrap around because last_digit < 10
            last_digit as u8
        }
    };
}

macro_rules! impl_get_last_digit_base_10_as_u8_for_prim_unsigned_int {
    () => {
        fn get_last_digit_base_10_as_u8(&self) -> u8 {
            let rem: Self = self % 10;
            // cast doesn't wrap around because last_digit < 10
            rem as u8
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
    ($trait:path, @PRIM_UNSIGNED_INTS, $fn_macro_name:ident) => {
        impl_trait!($trait, [u8,u16,u32,u64,u128,usize], $fn_macro_name);
    };
    ($trait:path, @PRIM_SIGNED_INTS, $fn_macro_name:ident) => {
        impl_trait!($trait, [i8,i16,i32,i64,i128,isize], $fn_macro_name);
    };
    ($trait:path, [$($t:ty),+], $fn_macro_name:ident) => {
        $(
            impl_trait_for_t!($trait, $t, $fn_macro_name);
        )+
    };
}

impl_trait!(
    crate::num_sys::base_10::GetLastDigitBase10AsU8,
    @PRIM_UNSIGNED_INTS,
    impl_get_last_digit_base_10_as_u8_for_prim_unsigned_int
);
impl_trait!(
    crate::num_sys::base_10::GetLastDigitBase10AsU8,
    @PRIM_SIGNED_INTS,
    impl_get_last_digit_base_10_as_u8_for_prim_signed_int
);

// TODO: decompose the function and maybe share the results with the library authors
// TODO: track https://github.com/rust-num/num-bigint/issues/233
#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
impl crate::num_sys::base_10::GetLastDigitBase10AsU8 for num_bigint::BigUint {
    fn get_last_digit_base_10_as_u8(&self) -> u8 {
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
impl crate::num_sys::base_10::GetLastDigitBase10AsU8 for num_bigint::BigInt {
    fn get_last_digit_base_10_as_u8(&self) -> u8 {
        self.magnitude().get_last_digit_base_10_as_u8()
    }
}