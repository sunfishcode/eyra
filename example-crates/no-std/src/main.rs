#![no_std]
#![no_main]

extern crate eyra;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}
