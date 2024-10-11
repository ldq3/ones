pub mod cpu;
mod memory;
mod process;
mod arch;

pub use arch::riscv64 as arch_ins; // architecture instance

pub fn init() {
    clear_bss();
    cpu::init();
    memory::heap::init_heap();
    memory::heap::test_heap();
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