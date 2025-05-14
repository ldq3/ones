/*!
public for test

# 函数
init()

# 结构体
Frame
*/
#[derive(Clone)]
pub struct Frame {
    pub number: usize
}

impl Drop for Frame {
    fn drop(&mut self) {
        let mut allocator = ALLOCATOR.lock();
        let allocaor = allocator.as_mut().unwrap();
        info!("Frame {} is deallocated.", self.number);
        allocaor.dealloc(self.number)
    }
}

impl Frame {
    /**
    Allocate a new frame, panic if it fails.
    */
    #[inline]
    pub fn new() -> Self {
        let mut allocator = ALLOCATOR.lock();
        let allocaor = allocator.as_mut().unwrap();
        Self {
            number: allocaor.alloc().unwrap()
        }
    }
    /**
    Allocate n contiguous frames, return.
    */
    pub fn new_contig(n: usize) -> Vec<Self> {
        let mut allocator = ALLOCATOR.lock();
        let allocaor = allocator.as_mut().unwrap();

        let base = allocaor.alloc_contig(n)
        .expect("Frame allocator over.");

        let mut vecotr = Vec::new();
        for i in 0..n {
            vecotr.push(Frame { number: base + i });
        }

        vecotr
    }
    /**
    Initialize the frame allocator.
    */
    #[inline]
    pub fn init(head: usize, tail: usize) {
        let mut allocator = ALLOCATOR.lock();
        if let None = *allocator {
            *allocator = Some(Allocator::new(head, tail).unwrap())
        }
    }
}

use alloc::vec::Vec;
use log::info;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::Allocator;

lazy_static! {
    static ref ALLOCATOR: Mutex<Option<Allocator>> = Mutex::new(None);
}
