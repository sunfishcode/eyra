#![no_std]
#![no_main]
#![feature(rustc_private)]
#![feature(lang_items)]
#![allow(internal_features)]

extern crate libc;
extern crate eyra;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
