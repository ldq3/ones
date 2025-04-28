/*!
对于线程的话，除了主线程仍然从程序入口点（一般是 main 函数）开始执行之外，每个线程的生命周期都与程序中的一个函数的一次执行绑定。也就是说，线程从该函数入口点开始执行，当函数返回之后，线程也随之退出。因此，在创建线程的时候我们需要提供程序中的一个函数让线程来执行这个函数。

我们用 C 语言中的线程 API 来举例说明。在 C 语言中，常用的线程接口为 pthread 系列 API，这里的 pthread 意为 POSIX thread 即 POSIX 线程。这组 API 被很多不同的内核实现所支持，基于它实现的应用很容易在不同的平台间移植。

一个线程访问另一个线程的栈这种行为并不会被操作系统和硬件禁止

当 Trap 控制流准备调用 __switch 函数使任务从运行状态进入暂停状态的时候，让我们考察一下它内核栈上的情况，在准备调用 __switch 函数之前，内核栈上从栈底到栈顶分别是保存了应用执行状态的 Trap 上下文以及内核在对 Trap 处理的过程中留下的调用栈信息。由于之后还要恢复回来执行，我们必须保存 CPU 当前的某些寄存器，我们称它们为 任务上下文 (Task Context)。

任务上下文保存的位置，在任务管理器 TaskManager ，在里面能找到一个数组 tasks ，其中的每一项都是一个任务控制块即 TaskControlBlock ，它负责保存一个任务的状态，而任务上下文 TaskContext 被保存在任务控制块中。在内核运行时我们会初始化 TaskManager 的全局实例 TASK_MANAGER ，因此所有任务上下文实际保存在在 TASK_MANAGER 中，从内存布局来看则是放在内核的全局数据 .data 段中。当我们将任务上下文保存完毕之后则转化为下图右侧的状态。当要从其他任务切换回来继续执行这个任务的时候，CPU 会读取同样的位置并从中恢复任务上下文。
*/

pub mod context;

use context::Context;

pub trait Thread {
    /**
    Thread id allocator is holded by 
    */
    fn new(pid: usize, tid: usize) -> Self;
    fn empty() -> Self;
    /**
    thread clone
    */
    fn clone(&self) -> Self;
}

/**
state
*/
pub struct ModelThread {
    pub pid: usize,
    pub tid: usize,

    pub context: Context,
}

impl ModelThread {
    #[inline]
    pub fn new(pid: usize, tid: usize) -> Self {
        let context = Context::empty();
        
        ModelThread {
            pid,
            tid,
            context,
        }
    }
}
