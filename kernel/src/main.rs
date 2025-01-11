#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![allow(unused)] // #FIXME

mod runtime;
extern crate alloc; // FIXME: 为什么这样写？

// core
mod inner;
mod outer;
mod concurrency;
mod file_system;

// assist
mod logger;

#[no_mangle]
pub fn kernel_main() -> ! {
    logger::init();
    
    inner::init();

    panic!("Shutdown machine!");
}
