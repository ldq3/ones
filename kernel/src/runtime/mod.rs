pub mod lang_items;
pub mod heap;

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

static mut HEAP_SPACE: [u8; config::KERNEL_HEAP_SIZE] = [0; config::KERNEL_HEAP_SIZE];

pub fn init() {
    clear_bss();

    // initialize heap
    unsafe {
        heap::HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, config::KERNEL_HEAP_SIZE);
    }
}

mod config {
    pub const KERNEL_HEAP_SIZE: usize = 0x800_000;
}

pub mod test {
    use super::heap;

    pub fn main() {
        heap::test::main();
    }
}