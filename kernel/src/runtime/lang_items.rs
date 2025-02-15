use core::panic::PanicInfo;
use core::arch::global_asm;

use crate::println;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("paniced at {}: {} {}", location.file(), location.line(), info.message());
    } else {
        println!("paniced: {}", info.message());
    }

    use crate::virtualization::cpu::*;
    Handler::shutdown(true)
}
