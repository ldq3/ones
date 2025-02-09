#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

mod runtime;
extern crate alloc; // FIXME: 为什么这样写？

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

// ekernel, MEMORY_END
