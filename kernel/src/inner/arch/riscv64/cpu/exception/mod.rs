use core::arch::global_asm;
use crate::inner::cpu::exception::{
    Context as TrapContext,
    HandlerTrait as TrapHandler,
};
use log::info;
use riscv::register::{
    scause::{self, Interrupt, Exception, Trap},
    sepc,
    sstatus::{ self, Sstatus, SPP },
    stvec::{ self, TrapMode },
    sie,
};
use crate::println;

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

pub struct Context {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

global_asm!(include_str!("context.S"));

impl TrapContext for Context { 
    fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    fn inc_epc(&mut self, n: usize) {
        self.sepc += n;
    }

    // function support
    fn set_ret(&mut self, ret: usize) {
        self.x[10] = ret;
    }

    fn fn_args(&self) -> [usize; 3] {
        [ self.x[10], self.x[11], self.x[12] ]
    }

    fn syscall_id(&self) -> usize {
        self.x[17]
    }
}

pub struct HandlerRv;

use crate::inner::cpu::timer::HandlerTrait;
use crate::inner::process::address_space::config::CONTEXT_ADDR;

impl TrapHandler<Context> for HandlerRv {
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
            core::ptr::read(CONTEXT_ADDR as *const Context)
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
                crate::inner::cpu::timer::Handler::set_next_trigger();
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
    fn get_kernel_context() {
       
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}
