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

use arch_ins::exception::Handler as TrapHandler;
use riscv::asm::ebreak;
use crate::exception::Handler;

global_asm!(include_str!("arch/riscv/entry.asm"));

#[no_mangle]
pub fn kernel_main() -> ! {
    clear_bss();
    
    logger::init();
    
    TrapHandler::init();
    unsafe{
        ebreak();
    }

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
