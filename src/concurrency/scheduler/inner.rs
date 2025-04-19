use alloc::{ 
    collections::vec_deque::VecDeque,
    vec::Vec
};
use crate::concurrency::scheduler::Process;

pub trait Scheduler {
    fn new() -> Self;
    /**
    在 idle 内核控制流中使用，保存当前内核控制流上下文，并切换至由 (pid, tid) 指定的用户程序内核 intervene 控制流。
    */
    fn switch_to_ready(&mut self);
    /**
    由当前用户程序内核 intervene 控制流切换至 idle 控制流
    */
    fn switch_to_idle(&mut self);
}

pub trait Dependence {
    fn switch(current: usize, next: usize);
}

/**
ready: (process id, thread id)
*/
pub struct ModelScheduler<P: Process> {
    pub process: Vec<P>,
    pub ready: VecDeque<(usize, usize)>,
    pub running: (usize, usize)
}

impl<P: Process> ModelScheduler<P> {
    pub fn new() -> Self {
        let mut process = Vec::new();
        process.push(Process::new_kernel());

        ModelScheduler {
            process,
            ready: VecDeque::new(),
            running: (0, 0)
        }
    }

    pub fn new_process(&mut self, elf: &[u8]) {
        let process = P::new(elf);
        let pid = process.id();
        self.process.insert(pid, process);
        self.ready.push_back((pid, 0));
    }

    pub fn fork(&mut self) {
        let (pid, _) = self.running;
        let process = &mut self.process[pid];
        let child = process.fork();
        self.process.insert(child.id(), child);
        self.ready.push_back((pid, 0));
    }

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

    pub fn switch_to_idle(&mut self) -> (usize, usize) {
        let (pid, tid) = self.running;

        let process = &mut self.process[pid];
        let current = &mut process.get_context_mut(tid);
        let current = current as *mut _ as usize;

        let kernel = &self.process[0];
        let idle = &kernel.get_context_ref(0) as *const _ as usize;

        (current, idle)
    }
}