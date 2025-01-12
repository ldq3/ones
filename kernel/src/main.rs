#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![allow(unused)] // #FIXME

extern crate alloc; // FIXME: 为什么这样写？

mod runtime;
mod inner;
mod outer;
mod concurrency;
mod file_system;

mod logger;

#[no_mangle]
pub fn kernel_main() -> ! {
    runtime::init();
    logger::init();
    inner::init();

    test::main();

    panic!("Shutdown machine!");
}

mod test {
    use super::{
        runtime,
        logger,
    };

    pub fn main() {
        runtime::test::main();
        logger::test::main();
    }
}