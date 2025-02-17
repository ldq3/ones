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
mod context;

use log::info;

use ones::virtualization::memory::page::AddressTrait;
use riscv::register::{self, sscratch};

use core::arch::global_asm;
global_asm!(include_str!("handler.S"));

use context::Context;

pub trait Exception {
    fn init();

    #[allow(unused)]
    fn distribute_in_user();

    #[allow(unused)]
    fn distribute_in_kernel(context: &mut Context);
}

pub struct Handler;

use register::{ scause::{ self, Trap, Exception::*, Interrupt::* }, stval, sepc };
impl Exception for Handler {
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
            sscratch::write(Self::distribute_in_kernel as usize);

            sstatus::set_sie(); // enable interrupt

            // sie::set_stimer(); // enable timer interrupt
        }
        
        info!("distribute_in_kernel: {:x}", Self::distribute_in_kernel as usize);

        // info!("Testing exception handler.");
        test::main();
    }

    fn distribute_in_user() {
        let mut context = Context::new();

        let scause = scause::read();
        // let stval = stval::read(),
        let sepc = sepc::read();
        info!("trap: cause: {:?}, epc: 0x{:#x}", scause.cause(), sepc);

        match scause.cause() {
            Trap::Exception(Breakpoint) => {
                info!("a breakpoint set @0x{:x}", context.sepc);
                context.sepc += 2;
            },
            Trap::Interrupt(SupervisorTimer) => {
                use crate::peripheral::timer::{ self, Timer };
                timer::Handler::set_next_trigger();
            }
            _ => {
                info!("unsupported exception");
            }
        }
    }

    fn distribute_in_kernel(context: &mut Context) {
        info!("Distribute in kernel.");

        let scause = scause::read();
        let stval = stval::read();
        match scause.cause() {
            Trap::Interrupt(SupervisorTimer) => {
                use crate::peripheral::timer::{ self, Timer };
                timer::Handler::set_next_trigger();
                timer::Handler::check();
                info!("Timer.");
            },
            Trap::Exception(Breakpoint) => {
                info!("a breakpoint set @0x{:x}", context.sepc);
                context.sepc += 2;
            },
            _ => {
                panic!(
                    "Unsupported trap from kernel: {:?}, stval = {:#x}!",
                    scause.cause(),
                    stval
                );
            }
        }
    }
}

mod test {
    pub fn main() {
        use riscv::asm::ebreak;

        unsafe { ebreak(); }
    }
}