#![no_main]
#![no_std]

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod logger;
mod trap;

use core::arch::global_asm;
use log::info;

global_asm!(include_str!("entry.asm"));

const STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct Stack {
    space: [u8; STACK_SIZE],
}

impl Stack {
    fn get_sp(&self) -> usize {
        self.space.as_ptr() as usize + STACK_SIZE
    }
}
static KERNEL_STACK: Stack = Stack { space: [0; STACK_SIZE] };
static USER_STACK: Stack = Stack { space: [0; STACK_SIZE] };

#[no_mangle]
pub fn kernel_main() -> ! {
    clear_bss();
    logger::init();
    trap::init();
    
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