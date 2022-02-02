//! A module for compatibility macros.
//!
//! # Problem
//!
//! In Rust 1.57.0, negative trait bounds are not supported and
//! ```compile_fail
//! impl <T> IntSubset for T where T: FixedSizeIntSubset + !ArbitrarySizeIntSubset {}
//! impl <T> IntSubset for T where T: !FixedSizeIntSubset + ArbitrarySizeIntSubset {}
//! ```
//! will fail to compile even with `#![feature(negative_impls)]` enabled due to conflicting implementations
//! of trait [IntSubset][super::IntSubset] and, ultimately, lack of support for negative trait bounds.
//!
//! As a result, whenever [FixedSizeIntSubset][super::FixedSizeIntSubset] or
//! [ArbitrarySizeIntSubset][super::ArbitrarySizeIntSubset] is implemented for `T`,
//! one way or another [IntSubset][super::IntSubset] **must** be implemented for `T` as well.
//! Due to semantics of the trait, the type implementing [IntSubset][super::IntSubset] also **must**
//! implement either [FixedSizeIntSubset][super::FixedSizeIntSubset] or
//! [ArbitrarySizeIntSubset][super::ArbitrarySizeIntSubset].
//!
//! # Solution
//!
//! For compatibility, implementing either [FixedSizeIntSubset][super::FixedSizeIntSubset]
//! or [ArbitrarySizeIntSubset][super::ArbitrarySizeIntSubset] is strongly recommended via
//! the use of [impl_arbitrary_size_int_subset] or [impl_fixed_size_int_subset], respectively, both of
//! which will be deprecated in favor of
//! ```rust
//! # use dd_maths_traits::int::ArbitrarySizeIntSubset as ArbitrarySizeIntSubset;
//! # struct T;
//! impl ArbitrarySizeIntSubset for T {}
//! ```
//! and
//! ```rust
//! # use dd_maths_traits::int::FixedSizeIntSubset as FixedSizeIntSubset;
//! # struct T;
//! impl FixedSizeIntSubset for T {}
//! ```
//! once
//! ```compile_fail
//! impl <T> IntSubset for T where T: FixedSizeIntSubset + !ArbitrarySizeIntSubset {}
//! impl <T> IntSubset for T where T: !FixedSizeIntSubset + ArbitrarySizeIntSubset {}
//! ```
//! is possible at least in nightly Rust.

/// View the documentation for [this module][crate::int::compat].
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
pub macro impl_arbitrary_size_int_subset($t:ty) {
    impl super::ArbitrarySizeIntSubset for $t {}
    impl super::IntSubset for $t {}
}

/// View the documentation for [this module][crate::int::compat].
#[cfg(any(doc, test, doctest, feature = "decl_macro"))]
pub macro impl_fixed_size_int_subset($t:ty) {
    impl super::FixedSizeIntSubset for $t {}
    impl super::IntSubset for $t {}
}
