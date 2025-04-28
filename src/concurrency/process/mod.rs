pub mod thread;

use thread::context::Context;

use alloc::{ vec, vec::Vec };
use thread::Thread;

use crate::{
    runtime::address_space::AddressSpace,
    Allocator
};

/**
# 已有
new_pid()
*/
pub trait Process {
    fn new(elf_data: &[u8]) -> Self; // FIXME
    fn id(&self) -> usize;
    /**
    # 返回值
    thread id
    */
    fn spawn(&mut self, _entry: usize, _arg: usize) -> usize {
        0
    }

    fn new_kernel() -> Self;

    #[inline]
    fn new_pid() -> usize {
        ALLOCATOR.lock().alloc().unwrap()
    }

    fn fork(&mut self) -> Self;

    fn get_context_mut(&mut self, tid: usize) -> &mut Context;
    fn get_context_ref(&self, tid: usize) -> &Context;
}

pub struct ModelProcess<T: Thread, A: AddressSpace> {
    pub id: usize, // 如果没有该字段不方便实现 Drop

    pub address_space: A,
    pub thread: Vec<T>,
    
    pub parent: Option<usize>,
    pub children: Vec<usize>,

    /**
    Thread id allocator.
    */
    pub allocator: Allocator,
}

impl<T: Thread, A: AddressSpace> ModelProcess<T, A>  {
    pub fn new(pid: usize, elf_data: &[u8]) -> Self {
        let mut address_space = A::from_elf(elf_data);
        address_space.new_intervene(0);

        use crate::Allocator;
        let allocator = Allocator::new(1, 15).unwrap();

        let thread = vec![T::new(pid, 0)];
        
        ModelProcess {
            id: pid,
            address_space,
            thread,
            parent: None,
            children: Vec::new(),
            allocator
        }
    }
}

impl<T: Thread, A: AddressSpace> Drop for ModelProcess<T, A> {
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
