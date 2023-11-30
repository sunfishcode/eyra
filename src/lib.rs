#![allow(unused_extern_crates)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

// If enabled, re-export `std` so that we can be used as `std` to avoid the
// `extern crate eyra;`.
#[cfg(feature = "be-std")]
pub use std::*;

/// All the functionality of Eyra is factored out into separate libraries. This
/// `extern crate` line is needed to ensure that libraries that intercept C
/// library symbols get linked in.
extern crate c_gull;
