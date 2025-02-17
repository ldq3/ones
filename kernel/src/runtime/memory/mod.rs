mod heap;

pub trait Memory {
    fn init();
    fn clear_bss();
}

pub struct Handler;

impl Memory for Handler {
    fn init() {
        Self::clear_bss();

        use heap::Heap;
        heap::Handler::init();
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
}