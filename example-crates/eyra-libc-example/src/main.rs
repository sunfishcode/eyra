extern crate eyra;

fn main() {
    println!("Hello world using Rust `println!`!");
    unsafe { libc::printf("Hello world using libc `printf`!\n\0".as_ptr().cast()); }
}
