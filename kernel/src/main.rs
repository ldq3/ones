#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

mod lang_items;
mod logger;
mod inner;
mod outer;

extern crate alloc;

use core::arch::global_asm;
use log::info;

global_asm!(include_str!("inner/cpu/arch/riscv/entry.asm"));

#[no_mangle]
pub fn kernel_main() -> ! {
    clear_bss();
    
    logger::init();
    
    inner::init();

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
