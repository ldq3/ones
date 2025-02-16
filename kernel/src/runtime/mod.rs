pub mod lang_items;
pub mod heap;

pub trait Runtime {
    fn init();
}

pub struct Handler;

impl Runtime for Handler {
    fn init() {
        clear_bss();

        use heap::Heap;
        heap::Handler::init();
    }
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

mod config {
    
}

mod test {

}