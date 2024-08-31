#![no_main]
#![no_std]

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod logger;

use core::arch::global_asm;

use log::info;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn kernel_main() -> ! {
    clear_bss();
    logger::init();
    
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