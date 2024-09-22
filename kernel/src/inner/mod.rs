pub mod cpu;
mod memory;

pub fn init() {
    clear_bss();
    cpu::init();
    memory::init_heap();
    memory::test_heap();
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