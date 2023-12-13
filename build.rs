use std::{env, fmt::Display, path::Path};

extern crate pkg_config;

/// Based on the OS or target environment we are building for,
/// this function will return an expected default library linking method.
///
/// If we build for Windows, MacOS, or Linux with musl, we will link statically.
/// However, if you build for Linux without musl, we will link dynamically.
///
/// **Info**:
/// This is a helper-function and may not be called if
/// if the `static`-feature is enabled, the environment variable
/// `LIBSOXR_STATIC` or `SOXR_STATIC` is set.
fn default_library_linking() -> bool {
    #[cfg(any(windows, target_os = "macos", target_env = "musl"))]
    {
        true
    }
    #[cfg(any(target_os = "freebsd", all(unix, target_env = "gnu")))]
    {
        false
    }
}

fn is_static_build() -> bool {
    if cfg!(feature = "static") && cfg!(feature = "dynamic") {
        default_library_linking()
    } else if cfg!(feature = "static")
        || env::var("LIBSOXR_STATIC").is_ok()
        || env::var("SOXR_STATIC").is_ok()
    {
        println!("cargo:info=Static feature or environment variable found.");

        true
    } else if cfg!(feature = "dynamic") {
        println!("cargo:info=Dynamic feature enabled.");

        false
    } else {
        println!("cargo:info=No feature or environment variable found, linking by default.");

        default_library_linking()
    }
}

/// Outputs the library-file's prefix as word usable for actual arguments on
/// commands or paths.
const fn rustc_linking_word(is_static_link: bool) -> &'static str {
    if is_static_link {
        "static"
    } else {
        "dylib"
    }
}

fn build_soxr(is_static: bool) {
    let soxr_path = Path::new("soxr");

    println!(
        "cargo:info=Soxr source path used: {:?}.",
        soxr_path
            .canonicalize()
            .expect("Could not canonicalise to absolute path")
    );

    println!("cargo:info=Building Soxr via CMake.");
    let soxr_build_dir = cmake::Config::new("soxr")
        .profile("Release")
        .define("WITH_CR32S", "OFF")
        .define("BUILD_SHARED_LIBS", if is_static { "OFF" } else { "ON" })
        .define("WITH_OPENMP", "OFF")
        .build();
    link_soxr(is_static, soxr_build_dir.display())
}

fn link_soxr(is_static: bool, soxr_build_dir: impl Display) {
    let is_static_text = rustc_linking_word(is_static);

    println!(
        "cargo:info=Linking Soxr as {} lib: {}",
        is_static_text, soxr_build_dir
    );
    println!("cargo:rustc-link-lib={}=soxr", is_static_text);
    println!("cargo:rustc-link-search=native={}/lib", soxr_build_dir);
}

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        // do not probe for libsoxr when compiling at docs.rs
        if let Err(_e) = pkg_config::Config::new()
            .statik(is_static_build())
            .probe("soxr")
        {
            build_soxr(is_static_build());
        }
    }
}
