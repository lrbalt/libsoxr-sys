//! # libsoxr-sys
//! This crate is generated from [libsoxr](https://sourceforge.net/projects/soxr/)
//! using [bindgen](https://github.com/rust-lang/rust-bindgen).
//!
//! The documentation for this library can be found in the original C header
//! file [`soxr.h`](https://sourceforge.net/p/soxr/code/ci/master/tree/src/soxr.h) of libsoxr.

#![allow(non_camel_case_types)]
#![allow(clippy::redundant_static_lifetimes)]
include!("generated.rs");

pub type soxr_fn_state_t = *const std::os::raw::c_void;
pub type soxr_fn_state_t_mut = *mut std::os::raw::c_void;

// bindgen did not generate the following redefines
pub const SOXR_HQ: u32 = SOXR_20_BITQ; /* 'High quality'. */
pub const SOXR_VHQ: u32 = SOXR_28_BITQ; /* 'Very high quality'. */

#[test]
fn test_version() {
    use std::ffi::CStr;

    let version = unsafe { soxr_version() };
    let cstr = unsafe { CStr::from_ptr(version) };
    let version = std::str::from_utf8(cstr.to_bytes()).unwrap();
    println!("{}", version);
    assert_eq!("libsoxr-0.1.3", version);
}
