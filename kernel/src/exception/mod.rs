/*!
为什么不能用 call

符号 `<<` 的优先级

内核态产生中断时，将上下文压栈

用户态产生中断时，将上下文保存在某页中

Rust 会在函数的开始和结尾加入一些额外的指令，控制栈寄存器等

指令相对寻址与虚拟内存

问题：
- 修改 epc

sscratch 寄存器：user stack 和 user context

# 指令
`sfence.vma`：清除 TLB 缓存

# TODO
Trap::Interrupt(SupervisorExternal) => {
    crate::board::irq_handler();
}

Trap::Exception(Exception::UserEnvCall) => {
    cx.inc_epc(4);
    cx.set_ret(
        trap::syscall::syscall(cx.syscall_id(), cx.fn_args()) as usize
    );
},
*/
mod system_call;
mod context;

use log::info;

use ones::virtualization::memory::page::Address as _;
use riscv::register::{self, sscratch};

use core::arch::global_asm;
global_asm!(include_str!("handler.S"));

pub struct Handler;

use ones::exception::{ Exception, Cause };
use crate::exception::context::Context;
impl Exception<Context> for Handler {
    fn init() {
        use register::{ stvec::{ self, TrapMode }, sstatus }; // sie
        
        use ones::virtualization::memory::config::TRAP_TEXT;
        extern "C" {
            fn user_handler();
            fn kernel_handler();
        }

        info!("Physical address of kernel_handler: {:x}", kernel_handler as usize);

        use crate::virtualization::memory::page::VirtualAddress;
        let kernel_handler = kernel_handler as usize - user_handler as usize + VirtualAddress::address_of_number(TRAP_TEXT); // the virtual address of kernel exception handler

        info!("Virtual address of kernel_handler: {:x}", kernel_handler);

        unsafe {
            stvec::write(kernel_handler, TrapMode::Direct); 
            sscratch::write(Self::distribute_in_kernel::<system_call::Handler> as usize);

            sstatus::set_sie(); // enable interrupt

            // sie::set_stimer(); // enable timer interrupt
        }
        
        info!("distribute_in_kernel: {:x}", Self::distribute_in_kernel::<system_call::Handler> as usize);

        // info!("Testing exception handler.");
        test::main();
    }

    fn cause() -> Cause {
        Cause::Breakpoint
    }

    fn value() {
        
    }
}

/**
for user process
*/
use ones::exception::ModelStack;
use crate::virtualization::memory;
pub type _Stack = ModelStack<memory::Handler>;

mod test {
    pub fn main() {
        use riscv::asm::ebreak;

        unsafe { ebreak(); }
    }
}
