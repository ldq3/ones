use alloc::{ 
    collections::vec_deque::VecDeque,
    vec::Vec
};

use crate::{
    Allocator,
    concurrency::scheduler::Process,
};

pub trait Scheduler {
    fn new() -> Self;
    
    fn new_process(&mut self, elf: &[u8]);
}

/**
ready: (process id, thread id)
*/
pub struct ModelScheduler<P: Process> {
    pub process: Vec<P>,
    pub ready: VecDeque<(usize, usize)>,
    pub running: (usize, usize),
    allocator: Allocator,
}

impl<P: Process> ModelScheduler<P> {
    pub fn new() -> Self {
        let mut process = Vec::new();
        process.push(Process::new_kernel());

        ModelScheduler {
            process,
            ready: VecDeque::new(),
            running: (0, 0),
            allocator: Allocator::new(0, 255).unwrap()
        }
    }

    // pub fn new_process(&mut self, elf: &[u8], kernel_table: usize, ks_bottom: usize, frame_number: usize) {
    //     use crate::memory::Address;

    //     let process = P::new(elf);

    //     let kernel = &mut self.process[0];
    //     let ksp = Address::address(ks_bottom + 1) - 1;

    //     use crate::intervene::data::Data;

    //     // use riscv::register::sstatus::{ self, SPP };
    //     let data = Data::get_mut(frame_number);
    //     // data.data_reg = DataReg::empty();
    //     // data.data_reg.sp_set(usp);
    //     // let mut sstatus = sstatus::read();
    //     // sstatus.set_spp(SPP::User);

    //     // data.status = sstatus.bits();
    //     // data.pc = address_space.0.entry;
        
    //     // data.kernel_info.addr_trans = satp(kernel_table);
    //     // data.kernel_info.dist = intervene::Handler::service_user as usize;
    //     // data.kernel_info.sp = ksp;

    //     let pid = process.id();
    //     self.process.insert(pid, process);
    //     self.ready.push_back((pid, 0));
    // }

    pub fn fork(&mut self) {
        let (pid, _) = self.running;
        let process = &mut self.process[pid];
        let child = process.fork();
        self.process.insert(child.id(), child);
        self.ready.push_back((pid, 0));
    }

    /**
    在 idle 内核控制流中使用，保存当前内核控制流上下文，并切换至由 (pid, tid) 指定的用户程序内核 intervene 控制流。
    */
    #[inline]
    pub fn switch_to_ready(&mut self) -> (usize, usize) {
        let kernel = &mut self.process[0];
        let idle = &mut kernel.get_context_mut(0);
        let idle = idle as *mut _ as usize;

        let (pid, tid) = self.ready.pop_back().unwrap();
        self.running = (pid, tid);
        let process = &self.process[pid];
        let next = process.get_context_ref(tid) as *const _ as usize;

        (idle, next)
    }

    /**
    由当前用户程序内核 intervene 控制流切换至 idle 控制流
    */
    pub fn switch_to_idle(&mut self) -> (usize, usize) {
        let (pid, tid) = self.running;

        let process = &mut self.process[pid];
        let current = &mut process.get_context_mut(tid);
        let current = current as *mut _ as usize;

        let kernel = &self.process[0];
        let idle = &kernel.get_context_ref(0) as *const _ as usize;

        (current, idle)
    }
    /**
    Allocate kernel stack for thread, return the address of kernel stack bottom in kernel address space.
    */
    pub fn alloc_kernel_stack(&mut self) -> (usize, usize) {
        use crate::runtime::address_space::config::INTERVENE_TEXT;

        let id = self.allocator.alloc().unwrap();
        let start = INTERVENE_TEXT - id * config::STACK_SIZE - 1;
        let end = start + config::STACK_SIZE - 1;

        (start, end)
    }
}

mod config {
    /// 单位：页
    pub const STACK_SIZE: usize = 1;
}