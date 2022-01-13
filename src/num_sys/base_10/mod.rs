pub mod int;

#[cfg(any(doc, test, doctest, feature = "std"))]
use std::string::ToString;
#[cfg(any(doc, test, doctest, feature = "std"))]
use num_traits::PrimInt;

#[cfg(any(doc, test, doctest, feature = "std"))]
pub trait ToCanonicalRepresentationBase10AsString {
    fn to_canonical_representation_base_10_as_string(&self) -> String;
}

#[cfg(any(doc, test, doctest, feature = "std"))]
impl<T> ToCanonicalRepresentationBase10AsString for T
where T: PrimInt + ToString {
    fn to_canonical_representation_base_10_as_string(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::num_sys::base_10::ToCanonicalRepresentationBase10AsString;

    #[test]
    pub fn to_canonical_representation_base_10_as_string_works_for_negative_primitive_signed_integers() {
        assert_eq!((-17).to_canonical_representation_base_10_as_string(), "-17");
    }
}