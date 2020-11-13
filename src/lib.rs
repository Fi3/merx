#![feature(const_fn)]
#![feature(const_panic)]
#![feature(const_generics)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[macro_use]
pub mod fixed;

#[macro_use]
pub mod asset;

pub mod utils;

pub use asset::{Asset, Credit, Debt};
