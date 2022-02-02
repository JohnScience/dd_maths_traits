// The module must be consistent with
// Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive
// https://doc.rust-lang.org/std/ops/index.html#structs

// Analogue of https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html but
// without `exhausted: bool` field
pub struct Bounds<T> {
    pub lower_bound: T,
    pub upper_bound: T,
}

#[derive(PartialEq, Eq)]
pub enum BoundKind {
    Open,
    Closed,
    Unbounded,
}

#[derive(PartialEq, Eq)]
pub struct BoundsKinds {
    lower_bound_kind: BoundKind,
    upper_bound_kind: BoundKind,
}

#[cfg_attr(docsrs, doc(cfg(feature = "adt_const_params")))]
#[cfg(any(doc, test, doctest, feature = "adt_const_params"))]
pub struct Range<T, const BOUNDS_KINDS: BoundsKinds>(Bounds<T>);

#[cfg(any(doc, test, doctest, feature = "adt_const_params"))]
const CLOSED_INTVL_BOUNDS_KINDS: BoundsKinds = BoundsKinds {
    lower_bound_kind: BoundKind::Closed,
    upper_bound_kind: BoundKind::Closed,
};

#[cfg_attr(docsrs, doc(cfg(feature = "adt_const_params")))]
#[cfg(any(doc, test, doctest, feature = "adt_const_params"))]
pub type ClosedIntvl<T> = Range<T, CLOSED_INTVL_BOUNDS_KINDS>;
