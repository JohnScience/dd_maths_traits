#[cfg(any(doc, test, doctest, feature = "std"))]
use {
    max_len_base_10_as_usize::MaxLenBase10AsUsize,
    is_signed_trait::IsSigned,
};

use std::ops::Rem;

#[cfg(any(doc, test, doctest, feature = "bigint"))]
use num_bigint::{BigUint, BigInt};

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(any(doc, test, doctest, feature = "std"))]
pub trait NewVecU8OfDigitsBase10Le {
    fn new_vec_u8_of_digits_le(&self) -> Vec<u8>;   
}

#[cfg(any(doc, test, doctest, feature = "std"))]
macro_rules! impl_new_vec_u8_of_digits_le {
    ($fn_name:ident) => {
        // TODO: consider implementing via sparse_capacity_mut + set_len
        fn $fn_name(&self) -> Vec<u8> {
            let mut vec =
                Vec::<u8>::with_capacity(Self::MAX_LEN_BASE_10_AS_USIZE - if Self::IS_SIGNED { 1 } else { 0 });
            let mut q = *self;
            loop {
                let (new_q, r) = (q / 10, q % 10);
                q = new_q;
                vec.push(
                    if Self::IS_SIGNED {
                        (r as i8).abs() as u8
                    } else {
                        r as u8
                    }
                );
                if q == 0 { break; };
            }
            vec
        }
    };
}

macro_rules! impl_trait {
    ($trait_name:ident::$fn_name:ident for @PRIM_INTS as $macro_name:ident) => {
        impl_trait!(
            $trait_name::$fn_name for [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize] as $macro_name
        );
    };
    ($trait_name:ident::$fn_name:ident for [$($t:ty),+] as $macro_name:ident) => {
        $(
            #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
            #[cfg(any(doc, test, doctest, feature = "std"))]
            impl $trait_name for $t {
                $macro_name!($fn_name);
            }
        )+
    }
}

impl_trait!(NewVecU8OfDigitsBase10Le::new_vec_u8_of_digits_le for @PRIM_INTS as impl_new_vec_u8_of_digits_le);

#[cfg(any(doc, test, doctest, all(feature = "bigint", feature = "std")))]
impl NewVecU8OfDigitsBase10Le for BigUint {
    #[inline(always)]
    fn new_vec_u8_of_digits_le(&self) -> Vec<u8> {
        self.to_radix_le(10)
    }
}

#[cfg(any(doc, test, doctest, all(feature = "bigint", feature = "std")))]
impl NewVecU8OfDigitsBase10Le for BigInt {
    #[inline(always)]
    fn new_vec_u8_of_digits_le(&self) -> Vec<u8> {
        let (_, vec_of_digits) = self.to_radix_le(10);
        vec_of_digits
    }
}
