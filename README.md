# libsoxr-sys

Rust raw FFI bindings for [libsoxr](https://sourceforge.net/projects/soxr/) which is a "High quality, one-dimensional sample-rate conversion library".

This wrapper library is licensed the same as libsoxr itself: LGPLv2.

To avoid too long build times, the finished binding is committed into this repository.
If you need to regenerate it, run `regenerate_bindings.sh` (and have `bindgen` set up when you do so).

# Documentation

The documentation for this library can be found in the original C header file [`soxr.h`](https://sourceforge.net/p/soxr/code/ci/master/tree/src/soxr.h) of libsoxr.

# Thanks

Inspiration for the setup of this crate comes from [alsa-sys](https://github.com/diwic/alsa-sys)