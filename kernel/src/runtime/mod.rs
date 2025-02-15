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


pub fn init() {
    clear_bss();

    use heap::Heap;
    heap::Handler::init();
}

mod config {
    
}

pub mod test {
    use super::heap;

    pub fn main() {
        heap::test::main();
    }
}