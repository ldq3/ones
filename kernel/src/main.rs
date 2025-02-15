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
    runtime::init();
    logger::init();
    virtualization::init();

    test::main();

    panic!("Shutdown machine!");
}

mod test {
    use super::{
        runtime,
        logger,
        // virtualization,
    };

    pub fn main() {
        runtime::test::main();
        logger::test::main();
        // inner::test::main();
    }
}
