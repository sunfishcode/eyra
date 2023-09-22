fn main() {
    // Pass -nostartfiles to the linker, when Eyra is enabled.
    if std::env::var("CARGO_FEATURE_EYRA").is_ok() {
        println!("cargo:rustc-link-arg=-nostartfiles");
    }
}
