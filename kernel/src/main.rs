#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![allow(unused)] // #FIXME

// core
mod inner;
mod outer;
mod concurrency;

// assist
mod lang_items;
mod logger;
extern crate alloc;
#[macro_use]
extern crate bitflags;

use log::info;

#[no_mangle]
pub fn kernel_main() -> ! {
    logger::init();
    
    inner::init();

    info!("Hello World!");
    panic!("Shutdown machine!");
}
