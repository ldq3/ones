mod syscall;

use core::arch::global_asm;
use log::error;
use riscv::register::{
    scause::{ self, Exception, Trap }, sstatus::{self, Sstatus, SPP}, stval, stvec::{ self, TrapMode }
};

#[repr(C)]
pub struct TrapContext {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn user_init(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };

        cx.set_sp(sp);
        cx
    }
}

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" { fn __store_trap_context(); }
    unsafe {
        stvec::write(__store_trap_context as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    let cause = if let Trap::Exception(cause) = scause.cause() {
        cause
    } else {
        panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
    };
    
    match cause {
        Exception::UserEnvCall => {
            cx.sepc += 4;
            cx.x[10] = syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        },
        Exception::StoreFault | Exception::StorePageFault => {
            error!("[kernel] PageFault in application, kernel killed it.");
        },
        Exception::IllegalInstruction => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
        },
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }

    cx
}