//! Integer traits.
//!
//! Many (if not all) traits provided by this module are
//! [marker traits](https://blog.logrocket.com/rust-traits-a-deep-dive/#:~:text=marker%20traits).
//!
//! # What are marker traits?
//!
//! Marker traits, such as [core::marker::Sized] or [core::marker::Sync], are the traits that
//! don't have any behaviour (=shared methods) but give the compiler (and the developers)
//! certain guarantees. Such design choice allows to disambiguate the intended behavior for
//! methods of otherwise ambiguous traits. For example, view the documentation of
//! [num_sys::GetLastDigitAsU8][crate::num_sys::GetLastDigitAsU8].

pub mod compat;

/// A [marker trait](https://blog.logrocket.com/rust-traits-a-deep-dive/#:~:text=marker%20traits)
/// indicating that the type models some subset of integers that is representable on the computer
/// (possibly, depending on the available resources of the given computer), i.e. either
/// [FixedSizeIntSubset] or [ArbitrarySizeIntSubset].
///
/// Notice that there might be no intuitive notion of
/// * "next" element in the subset;
/// * [additive inverse](https://en.wikipedia.org/wiki/Additive_inverse);
/// * or anything that one can expect from an integer type;
/// and that there are no guarantees that the [inhabitants](https://alexknvl.com/posts/counting-type-inhabitants.html)
/// of the type can be
/// * represented uniquely;
/// * queried for either [multiplicative](https://www.merriam-webster.com/dictionary/multiplicative%20identity)
/// or [additive](https://en.wikipedia.org/wiki/Additive_identity) identity;
/// * checked for being either identity;
/// * believed to correspond to the same integers regardless of some state;
/// * assumed to contain any strictly positive elements, any strictly negative elements, or zero.
///
/// However, IF addition and multiplication are defined, additive identity of the implementer must also be the
/// [asbosring element](https://en.wikipedia.org/wiki/Absorbing_element) for multiplication.
///
/// Reading the [int::compat][crate::int::compat] module docs before implementation is **required**.
pub trait IntSubset {}

/// A [marker trait](https://blog.logrocket.com/rust-traits-a-deep-dive/#:~:text=marker%20traits)
/// indicating that the type models some subset of integers s.t. each of them has size (not necessarily
/// on the stack) known at compile-time and identical for all inhabitants of the type.
///
/// Notice that the combination of [core::marker::Sized] and [IntSubset] does not guarantee
/// that the type implements [FixedSizeIntSubset] because [core::marker::Sized] takes into account
/// only [shallow size](https://dzone.com/articles/shallow-retained-and-deep-size).
///
/// ```rust
/// # struct T;
/// # fn main() {
/// assert_eq!(core::mem::size_of::<Box<T>>(), core::mem::size_of::<*const T>());
/// # }
/// ```
///
/// Reading the [int::compat][crate::int::compat] module docs before implementation is **required**.
pub trait FixedSizeIntSubset: Sized {}

#[cfg_attr(docsrs, doc(cfg(feature = "negative_impls")))]
#[cfg(any(doc, test, doctest, feature = "negative_impls"))]
// [Negative impl](https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html)
// is used to convey the [dichotomy](https://en.wikipedia.org/wiki/Dichotomy) of the [IntSubset] trait.
impl<T: FixedSizeIntSubset> !ArbitrarySizeIntSubset for T {}

// Implementation of the trait via generic impl block doesn't play nicely with
// negative impls because currently (Rust 1.57.0) there is no way to provide guarantee
// that T doesn't implement ArbitrarySizeIntSubset. At the time of writing, negative trait
// bounds are unavailable.
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(i8);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(u8);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(i16);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(u16);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(i32);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(u32);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(i64);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(u64);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(i128);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(u128);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(isize);
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
compat::impl_fixed_size_int_subset!(usize);

/// A [marker trait](https://blog.logrocket.com/rust-traits-a-deep-dive/#:~:text=marker%20traits)
/// indicating that the type models some subset of integers with the
/// [shallow, retained, or deep size](https://dzone.com/articles/shallow-retained-and-deep-size)
/// unknown at compile-time.
///
/// Reading the [int::compat][crate::int::compat] module docs before implementation is **required**.
pub trait ArbitrarySizeIntSubset {}

#[cfg_attr(docsrs, doc(cfg(feature = "negative_impls")))]
#[cfg(any(doc, test, doctest, feature = "negative_impls"))]
impl<T: ArbitrarySizeIntSubset> !FixedSizeIntSubset for T {}
