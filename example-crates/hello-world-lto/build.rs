fn main() {
    // Pass -nostartfiles to the linker.
    println!("cargo:rustc-link-arg=-nostartfiles");
}
