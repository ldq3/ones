/*!
每个线程的生命周期都与程序中的一个函数的一次执行绑定，线程从该函数入口点开始执行，当函数返回之后，线程也随之退出

一个线程访问另一个线程的栈这种行为并不会被操作系统和硬件禁止

当 Trap 控制流准备调用 __switch 函数使任务从运行状态进入暂停状态的时候，让我们考察一下它内核栈上的情况，在准备调用 __switch 函数之前，内核栈上从栈底到栈顶分别是保存了应用执行状态的 Trap 上下文以及内核在对 Trap 处理的过程中留下的调用栈信息。由于之后还要恢复回来执行，我们必须保存 CPU 当前的某些寄存器，我们称它们为任务上下文 (Task Context)。

# 接口标准
在 C 语言中，常用的线程接口为 pthread 系列 API，这里的 pthread 意为 POSIX thread 即 POSIX 线程
*/

pub mod context;

use alloc::vec::Vec;
use context::Context;
use crate::memory::Address;

pub trait Lib {
    /**
    # 返回值
    thread id
    */
    fn new(pid: usize) -> usize {
        let frame = Frame::new();

        let address = Address::address(frame.number); 
        let context = unsafe{ &mut *(address as *mut Context) };

        let tid = access(|scheduler| {
            let tid = scheduler.id.add();

            let thread = Thread {
                pid,
                tid,
                context
            };

            scheduler.thread[tid] = thread;
            // kernel map

            tid
        });

        tid
    }
}

/**
state
*/
pub struct Thread {
    pub pid: usize,
    pub tid: usize,

    pub context: &'static mut Context,
}

impl Thread {
    // fn access<F, V>(f: F) -> V 
    // where
    //     F: FnOnce(&mut T) -> V,
    // {
    //     let mut guard = SCHEDULER.lock();
    //     let option = guard.as_mut();
    //     if let Some(scheduler) = option {
    //         f(scheduler)
    //     } else { panic!("The scheduler is not initialized."); }
    // }
}

    // /**
    // 在 idle 内核控制流中使用，保存当前内核控制流上下文，并切换至由 (pid, tid) 指定的用户程序内核 intervene 控制流。
    // */
    // #[inline]
    // pub fn switch_to_ready(&mut self) -> (usize, usize) {
    //     let kernel = &mut self.data[0];
    //     let idle = kernel.get_context_mut(0);
    //     let idle = idle as *mut _ as usize;

    //     let (pid, tid) = self.ready.pop_back().unwrap();
    //     self.running = (pid, tid);
    //     let process = &self.data[pid];
    //     let next = process.get_context_ref(tid) as *const _ as usize;

    //     (idle, next)
    // }

    // /**
    // 由当前用户程序内核 intervene 控制流切换至 idle 控制流
    // */
    // pub fn switch_to_idle(&mut self) -> (usize, usize) {
    //     let (pid, tid) = self.running;

    //     let process = &mut self.data[pid];
    //     let current = &mut process.get_context_mut(tid);
    //     let current = current as *mut _ as usize;

    //     let kernel = &self.data[0];
    //     let idle = &kernel.get_context_ref(0) as *const _ as usize;

    //     (current, idle)
    // }
    // /**
    // Allocate kernel stack for thread, return the address of kernel stack bottom in kernel address space.
    // */
    // pub fn alloc_kernel_stack(&mut self) -> (usize, usize) {
    //     use crate::runtime::address_space::config::INTERVENE_TEXT;

    //     let id = self.allocator.alloc().unwrap();
    //     let start = INTERVENE_TEXT - id * config::STACK_SIZE - 1;
    //     let end = start + config::STACK_SIZE - 1;

    //     (start, end)
    // }

struct Scheduler {
    thread: Vec<Thread>,
    id: Preemptive,
}

use lazy_static::lazy_static;
use spin::Mutex;

use crate::{concurrency::scheduler::Preemptive, memory::page::frame::Frame};
lazy_static! {
    static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(
        Scheduler {
            thread: Vec::new(),
            id: Preemptive::new(config::CAP)
        }
    );
}
/**
Access thread scheduler.
*/
#[inline]
fn access<F, V>(f: F) -> V
where
    F: FnOnce(&mut Scheduler) -> V,
{
    let mut mutex = SCHEDULER.lock();
    f(&mut mutex)
}

mod config {
    /// 单位：页
    // pub const STACK_SIZE: usize = 1;

    /// 容量（capacity）
    pub const CAP: usize = 64;
}