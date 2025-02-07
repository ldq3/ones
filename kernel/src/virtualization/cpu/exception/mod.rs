use log::info;
use riscv::register::{
    scause::{self, Interrupt, Exception, Trap},
    sepc,
    sstatus,
    stvec::{ self, TrapMode },
    sie,
};

pub struct KernelContext {
    satp: usize,
    sp: usize
}

impl KernelContextTrait for KernelContext {
    #[no_mangle]
    fn get() -> Self {
        KernelContext {
            satp: 0,
            sp: 0
        }
    }
}

use core::arch::global_asm;
global_asm!(include_str!("handler.S"));

pub struct Handler;

impl HandlerTrait for Handler {
    fn init() {
        unsafe {
            extern "C" { fn handler(cx_addr: usize); }
            stvec::write(handler as usize, TrapMode::Direct);
            sstatus::set_sie();

            // enable timer interrupt
            sie::set_stimer();
        }

        info!("init trap handler")
    }

    #[no_mangle]
    fn distribute() {
        use crate::virtualization::{
            cpu::context::Context,
            process::address_space::GLOBAL_DATA_PAGE_NUMBER,
        };

        let mut cx = unsafe {
            core::ptr::read(GLOBAL_DATA_PAGE_NUMBER as *const Context)
        };

        let scause = scause::read();
        // let stval = stval::read(),
        let sepc = sepc::read();
        info!("trap: cause: {:?}, epc: 0x{:#x}", scause.cause(), sepc);

        match scause.cause() {
            Trap::Exception(Exception::Breakpoint) => {
                info!("a breakpoint set @0x{:x}", cx.sepc);
                cx.sepc += 2;
            },
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                info!("time");
                use crate::virtualization::cpu::timer::*;
                Timer::set_next_trigger();
            }
            // Trap::Exception(Exception::UserEnvCall) => {
                // cx.inc_epc(4);
                // cx.set_ret(
                    // trap::syscall::syscall(cx.syscall_id(), cx.fn_args()) as usize
                // );
            // },
            _ => {
                info!("unsupported exception");
            }
        }
    }
}


pub trait KernelContextTrait {
    fn get() -> Self;
}

pub trait HandlerTrait {
    fn init();
    fn distribute();
}

pub mod test {
    pub fn main() {
        use riscv::asm::ebreak;

        unsafe { ebreak(); }
    }
}