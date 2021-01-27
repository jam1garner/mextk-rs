#![no_std]
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
)]

include!("bindings.rs");

/// Re-export of [mextk-libc](https://docs.rs/mextk-libc) for accesssing the subset of libc
/// supported.
pub use mextk_libc as libc;
