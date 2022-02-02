#![cfg_attr(not(any(doc, test, doctest, feature = "std")), no_std)]
#![cfg_attr(any(doc, test, doctest, feature = "adt_const_params"), feature(adt_const_params))]
#![cfg_attr(any(doc, test, doctest, feature = "negative_impls"), feature(negative_impls))]
#![cfg_attr(any(doc, test, doctest, feature = "decl_macro"), feature(decl_macro))]

#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
extern crate num_bigint;

pub mod ranges;
pub mod int;
pub mod num_sys;
