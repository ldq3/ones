/*!
initialize context
let mut sstatus = sstatus::read();
// set CPU privilege to User after trapping back
sstatus.set_spp(SPP::User);
let mut cx = Self {
    x: [0; 32],
    sstatus,
    sepc: entry,
    kernel_satp,
    kernel_sp,
    trap_handler,
};
cx.set_sp(sp);
cx

#TODO: 整理设计模式
优化：没有未能实现的功能，可以不要 trait
*/

use alloc::{ 
    collections::vec_deque::VecDeque, vec::Vec
};
use spin::Mutex;

use crate::{ 
    file_system::{ Flag, file::File }, 
    concurrency::process::Process,
};

/**
kernel process.
*/
pub trait Main<P: Process + 'static>: Dependence<P> {
    fn init() {
        let mut process = Vec::new();
        process.push(Process::new_kernel());

        let inner = Model {
            process,
            ready: VecDeque::new(),
            running: (0, 0)
        };

        let mut handler = Self::get_ref().lock();
        if let None = *handler {
            *handler = Some(inner);
        } else {
            panic!("Cannot reinitialize the scheduler.");
        }
    }
    /**
    参考实现：
    ```
    fn new_process(elf: &[u8]) {
        let address_space = AddressSpace::from_elf(elf);
        let process = Process::new(address_space);

        let scheduler = Self::get_mut();

        // trap context

        let process_id = process.id();
        scheduler.process.insert(process_id, process);
        scheduler.ready.push_back((process_id, 0));
     }
    ```
    */
    fn new_process(elf: &[u8]);
    /**
    参考实现：
    ```
    fn fork() {
        // assert_eq!(self.thread_count(), 1);

        let scheduler = Self::get_mut();

        let (process_id, _) = scheduler.running;
        let parent = &mut scheduler.process[process_id];
        
        let address_space = parent.address_space();

        // TODO: copy fd table

        let child = P::new(address_space);

        let child_id = child.id();
        parent.add_child(child_id);
        
        // create main thread of child process
        
        scheduler.process[child_id] = child;
    }
    ```
    */
    fn fork();
    /**
    参考实现：
    ```
    fn spawn(entry: usize, arg: usize) {
        let scheduler = Self::get_mut();
        let (pid, _) = scheduler.running;
        let process = &mut scheduler.process[pid];

        let tid = process.spawn(entry, arg);
        scheduler.ready.push_back((pid, tid));
    }
    ```
    */
    fn spawn(entry: usize, arg: usize);
    /**
    在 idle 内核控制流中使用，保存当前内核控制流上下文，并切换至由 (pid, tid) 指定的用户程序内核 intervene 控制流。
    */
    fn switch_to_ready();
    /**
    由当前用户程序内核 intervene 控制流切换至 idle 控制流
    */
    fn switch_to_idle();

    fn access<F, V>(f: F) -> V 
    where
        F: FnOnce(&mut Model<P>) -> V,
    {
        let mut mutex = Self::get_ref().lock();
        let option = mutex.as_mut();
        if let Some(scheduler) = option {
            f(scheduler)
        } else { panic!("The scheduler is not initialized."); }
    }
}

pub trait Dependence<P: Process + 'static> {
    fn open_file(name: &str, flag: Flag) -> Option<File>;
    fn get_ref() -> &'static Mutex<Option<Model<P>>>;
}

/**
ready: (process id, thread id)
*/
pub struct Model<P: Process> {
    pub process: Vec<P>,
    pub ready: VecDeque<(usize, usize)>,
    pub running: (usize, usize)
}
