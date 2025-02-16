#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod runtime;
mod virtualization;
mod peripheral;

mod logger;

#[no_mangle]
pub fn kernel_main() -> ! {
    use runtime::Runtime;
    runtime::Handler::init();

    logger::init();
    virtualization::init();

    panic!("Shutdown machine!");
}
