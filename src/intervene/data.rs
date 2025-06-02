/*!
当 Trap 控制流准备调用 __switch 函数使任务从运行状态进入暂停状态的时候，让我们考察一下它内核栈上的情况，在准备调用 __switch 函数之前，内核栈上从栈底到栈顶分别是保存了应用执行状态的 Trap 上下文以及内核在对 Trap 处理的过程中留下的调用栈信息。由于之后还要恢复回来执行，我们必须保存 CPU 当前的某些寄存器，我们称它们为 任务上下文 (Task Context)。

任务上下文保存的位置，在任务管理器 TaskManager ，在里面能找到一个数组 tasks ，其中的每一项都是一个任务控制块即 TaskControlBlock ，它负责保存一个任务的状态，而任务上下文 TaskContext 被保存在任务控制块中。在内核运行时我们会初始化 TaskManager 的全局实例 TASK_MANAGER ，因此所有任务上下文实际保存在在 TASK_MANAGER 中，从内存布局来看则是放在内核的全局数据 .data 段中。当我们将任务上下文保存完毕之后则转化为下图右侧的状态。当要从其他任务切换回来继续执行这个任务的时候，CPU 会读取同样的位置并从中恢复任务上下文。
*/

use crate::concurrency::thread::context::Context;

#[repr(C)]
pub struct Data {
    /// context
    pub cx: Context,
    /// kernel information
    pub ki: KernelInfo,
}

// impl Data {
//     #[inline]
//     pub fn get_mut(frame_number: usize) -> &'static mut Self {
//         use crate::memory::Address;
//         let address = Address::address(frame_number);
//         unsafe{ &mut *(address as *mut Self) }
//     }
// }

#[repr(C)]
pub struct KernelInfo {
    pub addr_trans: usize, 
    /// kernel stack pointer
    pub sp: usize,
    /// the address of service routine
    pub service: usize,
}
