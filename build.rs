use std::{path::Path, fmt::Display};

extern crate pkg_config;

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        // do not probe for libsoxr when compiling at docs.rs
        if let Err(_e) = pkg_config::probe_library("soxr") {
            build_soxr(true);
        }
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
                .define("BUILD_SHARED_LIBS", "OFF")
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