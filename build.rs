extern crate pkg_config;

fn main() {
    if let Err(e) = pkg_config::probe_library("soxr") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
                "Pkg-config failed - usually this is because libsoxr development headers are not installed.\n\n\
                For Mac users using brew: brew install libsoxr\n\n\
                For Debian/Ubuntu users:\n# apt-get install libsoxr0-dev\n\n\
                pkg_config details:\n{}",
                e
            ),
            _ => panic!("{}", e)
        }
    }
}