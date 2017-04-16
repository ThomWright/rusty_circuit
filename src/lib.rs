#![allow(unknown_lints)]
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_extern_crates,
        unused_import_braces, unused_qualifications)]
#![warn(stutter)]

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate rulinalg;

#[cfg(not(test))]
extern crate rulinalg;

extern crate specs;

pub mod elements;
pub mod solver;

pub type Delta = f32;
