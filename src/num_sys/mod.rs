//! Traits that pertain to some [numeral systems](https://en.wikipedia.org/wiki/Numeral_system).

/// The trait that indicates that the implementer has the canonical interpretation
/// of "last digit" that is guaranteed to be represented as [core::primitive::u8].
/// 
/// * For [IntSubset][crate::int::IntSubset], it is the last digit base 10.
/// * For decimals, the last digit can be specified for the possible or requested precision (base 10).
pub trait GetLastDigitAsU8 {
    fn get_last_digit_as_u8(&self) -> u8;
}

pub trait GetMaxLenBase10AsUsize {
    fn get_max_len_base_10_as_usize(&self)-> usize;
}

/// The trait that indicates that the implementer has the canonical interpretation
/// of "digits" and each of them is guaranteed to be represented as [core::primitive::u8].
/// 
/// The trait itself may not specify the order of digits. However,
/// 1. for primitive integers it is much simpler first to obtain least significant digits; 
/// 2. [big integers](https://en.wikipedia.org/wiki/Arbitrary-precision_arithmetic) are commonly stored as
/// [little endian](https://en.wikipedia.org/wiki/Endianness) sequences of primitive unsigned integers
/// 
/// The trait does not specify whether the length of provided vector matches its capacity
#[cfg(any(doc, test, doctest, feature = "std"))]
pub trait CreateVecU8OfDigits {
    fn create_vec_u8_of_digits(&self) -> std::vec::Vec<u8>;
}

pub mod base_10;