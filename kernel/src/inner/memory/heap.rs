pub const KERNEL_HEAP_SIZE: usize = 0x800_000;

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

mod tests {
    use crate::println;
    
    #[allow(unused)]
    pub fn test_heap() {
        use alloc::boxed::Box;
        use alloc::vec::Vec;
        extern "C" {
            fn sbss();
            fn ebss();
        }
        let bss_range = sbss as usize..ebss as usize;
        let a = Box::new(5);
        assert_eq!(*a, 5);
        assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
        drop(a);
        let mut v: Vec<usize> = Vec::new();
        for i in 0..500 {
            v.push(i);
        }
        for i in 0..500 {
            assert_eq!(v[i], i);
        }
        assert!(bss_range.contains(&(v.as_ptr() as usize)));
        drop(v);
        println!("heap_test passed!");
    }
}
