use alloc::vec::Vec;

use crate::{
    runtime::address_space::Lib,
    Allocator
};

/**
# 已有
new_pid()
*/
pub trait Mod<A: Lib> {
     fn fork(&mut self) -> usize {
        let _id = ALLOCATOR.lock().alloc().unwrap();

        0
    }

    /**
    在当前进程的上下文中创建一个新的子进程，并将新进程的代码和数据复制到子进程的内存空间中

    # 返回值
    process id
    */
    fn spawn(&mut self, _address_space: A) -> usize {
        let _id = ALLOCATOR.lock().alloc().unwrap();

        0
    }
}

pub struct Process<A: Lib> {
    pub id: usize, // 如果没有该字段不方便实现 Drop

    pub address_space: A,
    pub thread: Vec<usize>, // thread id
    
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<A: Lib> Process<A>  {
    pub fn new(pid: usize, elf_data: &[u8]) -> Self {
        let mut address_space = A::from_elf(elf_data);
        address_space.new_intervene(0);
        
        Process {
            id: pid,
            address_space,
            thread: Vec::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl<A: Lib> Drop for Process<A> {
    #[inline]
    fn drop(&mut self) {
        ALLOCATOR.lock().dealloc(self.id);
    }
}

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ALLOCATOR : Mutex<Allocator> = Mutex::new(
        Allocator::new(0, 15).unwrap()
    );
}

lazy_static! {

}
