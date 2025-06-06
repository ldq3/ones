/*!
每个线程的生命周期都与程序中的一个函数的一次执行绑定，线程从该函数入口点开始执行，当函数返回之后，线程也随之退出

一个线程访问另一个线程的栈这种行为并不会被操作系统和硬件禁止

当 Trap 控制流准备调用 __switch 函数使任务从运行状态进入暂停状态的时候，让我们考察一下它内核栈上的情况，在准备调用 __switch 函数之前，内核栈上从栈底到栈顶分别是保存了应用执行状态的 Trap 上下文以及内核在对 Trap 处理的过程中留下的调用栈信息。由于之后还要恢复回来执行，我们必须保存 CPU 当前的某些寄存器，我们称它们为任务上下文 (Task Context)。

# 接口标准
在 C 语言中，常用的线程接口为 pthread 系列 API，这里的 pthread 意为 POSIX thread 即 POSIX 线程
*/

pub mod context;

use alloc::vec;
use alloc::vec::Vec;
use crate::{ 
    concurrency::scheduler::Scheduler as S,
    intervene::data::Data,
    runtime::{ address_space::AddressSpace, Segment },
};

pub trait Lib {
    /**
    # 返回值
    thread id
    */
    fn new(pid: usize) -> usize; 
}

/**
不用保存协程的信息
*/
#[derive(Clone)]
pub struct Thread {
    pub pid: usize,
    pub tid: usize,

    pub idata: usize,
}

impl Thread {
    /**
    #参数
    - pid: 进程 id
    - frame_number: intervene data\intervene stack 所在页面的页框号
    - entry: 线程的入口地址
    - ki: kernel information

    todo: idata

    # 返回值
    (thread id, 动态段和内核段）

    0. user stack
    1. intervene stack
    2. kernel stack
    */
    pub fn new(pid: usize, ustack_size: usize, kstack_size: usize) -> (usize, Vec<Segment>) {
        let mut segement = Vec::new();

        let tid = access(|scheduler| { scheduler.id.add() });
        
        process::access(|manager| {
            let process = manager.process[pid].as_mut().unwrap();
            segement.push(process.address_space.stack(tid, ustack_size));
            segement.push(AddressSpace::idata(tid));

            let kernel = manager.process[0].as_mut().unwrap();
            segement.push(kernel.address_space.stack(tid, kstack_size));
        });

        let thread = Thread {
            pid,
            tid,
            idata: 0
        };

        access(|scheduler| {
            scheduler.thread[tid] = Some(thread);
        });

        (tid, segement)
    }

    #[inline]
    pub fn idata(&mut self) -> &'static mut Data {
        unsafe{ &mut *(self.idata as *mut Data) }
    }
}

pub struct Scheduler {
    pub thread: Vec<Option<Thread>>,
    pub id: S,
}

use lazy_static::lazy_static;
use spin::Mutex;

use super::process;

lazy_static! {
    static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(
        Scheduler {
            thread: vec![None; 16],
            id: S::new(config::CAP)
        }
    );
}
/**
Access thread scheduler.
*/
#[inline]
pub fn access<F, V>(f: F) -> V
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