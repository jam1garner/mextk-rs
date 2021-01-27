//! A library for writing mods for Super Smash Brothers Melee in Rust
#![no_std]

/// Re-export of [mextk-sys](https://docs.rs/mextk-sys) for accessing the raw bindings
pub use mextk_sys as sys;

/// Re-export of [mextk-libc](https://docs.rs/mextk-libc) for accesssing the subset of libc
/// supported.
pub use sys::libc;
