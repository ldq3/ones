pub mod thread;

use thread::context::Context;

use alloc::vec::Vec;
use thread::Thread;

use crate::{
    memory::{Address, Flag},
    runtime::address_space::AddressSpace, Allocator
};

/**
# 已有
new_pid()
*/
pub trait Process: Dependence {
    fn new(elf_data: &[u8]) -> Self;
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

pub trait Dependence {
    /**
    Allocate kernel stack for thread, return the address of kernel stack bottom in kernel address space.
   */
    fn alloc_kernel_stack() -> usize {
        use crate::runtime::address_space::config::INTERVENE_TEXT;

        let id = KERNEL_STACK_ALLOCATOR.lock().alloc().unwrap();
        let start = INTERVENE_TEXT - id * config::STACK_SIZE - 1;
        let end = start + config::STACK_SIZE - 1;

        Self::kernel_map_area((start, end), Flag::R | Flag::W);

        Address::address(end + 1) - 1
    }

    fn kernel_map_area(range: (usize, usize), flag: Flag);
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

lazy_static! {
    static ref KERNEL_STACK_ALLOCATOR : Mutex<Allocator> = Mutex::new(
        Allocator::new(0, 255).unwrap()
    );
}

mod config {
    /// 单位：页
    pub const STACK_SIZE: usize = 1;
}
