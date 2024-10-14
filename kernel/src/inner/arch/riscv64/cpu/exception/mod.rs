use crate::inner::cpu::exception;

use core::arch::global_asm;
use log::info;
use riscv::register::{
    scause::{self, Interrupt, Exception, Trap},
    sepc,
    sstatus,
    stvec::{ self, TrapMode },
    sie,
};
use crate::println;

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

pub struct KernelContext {
    satp: usize,
    sp: usize
}

impl exception::KernelContext for KernelContext {
    
}

global_asm!(include_str!("handler.S"));

pub struct Handler;

use crate::inner::cpu::timer::Timer;
use crate::inner::process::address_space::GLOBAL_DATA_PAGE_NUMBER;
use super::context::Context;

impl exception::Handler for Handler {
    type KernelContext = KernelContext;

    fn init() {
        extern "C" { fn handler(cx_addr: usize); }
        unsafe {
            stvec::write(handler as usize, TrapMode::Direct);
            sstatus::set_sie();
        }

        info!("init trap handler")
    }   

    #[no_mangle]
    fn distribute() {
        let mut cx = unsafe {
            core::ptr::read(GLOBAL_DATA_PAGE_NUMBER as *const Context)
        };

        let scause = scause::read();
        // let stval = stval::read(),
        let sepc = sepc::read();
        println!("trap: cause: {:?}, epc: 0x{:#x}", scause.cause(), sepc);

        match scause.cause() {
            Trap::Exception(Exception::Breakpoint) => {
                breakpoint(&mut cx.sepc);
            },
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                println!("time");
                crate::inner::arch_ins::cpu::timer::Timer::set_next_trigger();
            }
            // Trap::Exception(Exception::UserEnvCall) => {
                // cx.inc_epc(4);
                // cx.set_ret(
                    // trap::syscall::syscall(cx.syscall_id(), cx.fn_args()) as usize
                // );
            // },
            _ => {
                println!("unsupported exception");
            }
        }
    }
    
    #[no_mangle]
    fn get_kernel_context() -> KernelContext {
        KernelContext {
            satp: 0,
            sp: 0
        }
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}
