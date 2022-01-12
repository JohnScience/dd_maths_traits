#![cfg_attr(not(any(doc, test, doctest, feature = "std")), no_std)]
#![feature(negative_impls)]
#![feature(const_trait_impl)]
#![feature(decl_macro)]

#[cfg(any(doc, test, doctest, feature = "num-bigint"))]
extern crate num_bigint;

pub mod int;
pub mod num_sys;
