use core::panic::PanicInfo;
use crate::{println, arch::riscv::driver::cpu::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("paniced at {}: {} {}", location.file(), location.line(), info.message());
    } else {
        println!("paniced: {}", info.message());
    }

    shutdown(true)
}