use buddy_system_allocator::LockedHeap;

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

pub mod test {
    use crate::println;
    
    pub fn main() {
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