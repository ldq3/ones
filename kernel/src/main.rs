#![no_main]
#![no_std]

mod lang_items;
mod logger;
mod exception;
mod arch;
mod driver;

use core::arch::global_asm;
use log::info;
// arch instance
use arch::riscv as arch_ins;

global_asm!(include_str!("arch/riscv/entry.asm"));

#[no_mangle]
pub fn kernel_main() -> ! {
    clear_bss();
    
    logger::init();
    
    exception::init();
    driver::timer::init();

    info!("Hello World!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
