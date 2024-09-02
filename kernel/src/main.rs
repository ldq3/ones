#![no_main]
#![no_std]

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod logger;
mod trap;
mod arch;

use core::arch::global_asm;
use log::info;
// arch instance
use arch::riscv as arch_ins; 
use arch_ins::trap::Handler as TrapHandler;
use crate::trap::Handler;


global_asm!(include_str!("entry.asm"));

const STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
pub struct Stack {
    space: [u8; STACK_SIZE],
}

impl Stack {
    fn get_sp(&self) -> usize {
        self.space.as_ptr() as usize + STACK_SIZE
    }

    pub fn push_context(&self, cx: arch_ins::trap::Context) -> &'static mut arch_ins::trap::Context {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<arch_ins::trap::Context>()) as *mut arch_ins::trap::Context;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

pub static KERNEL_STACK: Stack = Stack { space: [0; STACK_SIZE] };
pub static USER_STACK: Stack = Stack { space: [0; STACK_SIZE] };

#[no_mangle]
pub fn kernel_main() -> ! {
    clear_bss();
    
    logger::init();
    
    TrapHandler::init();
    TrapHandler::into_user();

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